provider "aws" {
  region  = var.region
  profile = "timeboost-dev"

  allowed_account_ids = [local.account_id]

  default_tags {
    tags = {
      espresso-bench-run     = var.name
      espresso-bench-owner   = var.owner
      espresso-bench-expires = var.expires_at
      espresso-bench-git     = var.git_rev
    }
  }
}

# az and ami_id come from `aws-bench preflight`, which already picked an AZ offering every
# requested instance type and resolved the AMI of their architecture; this module only looks
# up that AZ's default subnet.
locals {
  account_id = "027574771971"
}

data "aws_vpc" "default" {
  default = true
}

data "aws_subnets" "default" {
  filter {
    name   = "vpc-id"
    values = [data.aws_vpc.default.id]
  }
  filter {
    name   = "availability-zone"
    values = [var.az]
  }
  filter {
    name   = "default-for-az"
    values = ["true"]
  }
}

locals {
  pinned_subnet = length(data.aws_subnets.default.ids) > 0 ? data.aws_subnets.default.ids[0] : ""
}

resource "aws_key_pair" "this" {
  key_name   = "espresso-bench-${var.name}"
  public_key = var.ssh_public_key
}

# No API port is opened: every HTTP service runs on ctl and the driver reads it over ssh.
resource "aws_security_group" "this" {
  name        = "espresso-bench-${var.name}"
  description = "espresso-bench fleet ${var.name}"
  vpc_id      = data.aws_vpc.default.id

  ingress {
    description = "ssh from the operator"
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = [var.operator_cidr]
  }

  ingress {
    description = "all traffic within the fleet"
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    self        = true
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
}

resource "aws_instance" "host" {
  for_each = var.hosts

  ami                    = var.ami_id
  instance_type          = each.value.instance_type
  key_name               = aws_key_pair.this.key_name
  vpc_security_group_ids = [aws_security_group.this.id]
  subnet_id              = local.pinned_subnet

  associate_public_ip_address          = true
  instance_initiated_shutdown_behavior = "terminate"

  # The EBS balance metrics of the query host are read in 60 s periods; basic monitoring
  # publishes AWS/EC2 every 300 s, which leaves short loads without a datapoint.
  monitoring = each.value.role == "query"

  metadata_options {
    http_tokens = "required"
  }

  user_data                   = file(each.value.user_data_path)
  user_data_replace_on_change = true

  root_block_device {
    volume_type           = "gp3"
    volume_size           = each.value.root_gb
    iops                  = each.value.root_iops
    throughput            = each.value.root_mbps
    delete_on_termination = true

    # Explicit because default_tags do not reliably reach root volumes.
    tags = {
      Name                = "${var.name}-${each.key}-root"
      espresso-bench-run  = var.name
      espresso-bench-role = each.value.role
    }
  }

  # Inline rather than aws_ebs_volume plus attachment: the volume is deleted with the instance
  # at the shutdown timer, so an abandoned fleet leaves no volume behind.
  dynamic "ebs_block_device" {
    for_each = each.value.role == "query" && var.pg_volume != null ? [var.pg_volume] : []

    content {
      device_name           = "/dev/sdf"
      volume_type           = "gp3"
      volume_size           = ebs_block_device.value.gb
      iops                  = ebs_block_device.value.iops
      throughput            = ebs_block_device.value.mbps
      delete_on_termination = true

      tags = {
        Name                = "${var.name}-${each.key}-pg"
        espresso-bench-run  = var.name
        espresso-bench-role = "pg"
      }
    }
  }

  tags = {
    Name                = "${var.name}-${each.key}"
    espresso-bench-role = each.value.role
  }

  lifecycle {
    precondition {
      condition     = local.pinned_subnet != ""
      error_message = "No default subnet for ${var.az} in ${var.region}."
    }
  }
}

# The RDS instance, its parameter group and the schedule that deletes it before the hosts end.
# The schedule is what bounds the cost of an rds fleet whose laptop is gone: hosts terminate
# themselves, RDS does not.
locals {
  rds_enabled     = var.rds != null
  fargate_enabled = var.fargate != null
  reaper_enabled  = local.rds_enabled || local.fargate_enabled
  rds_name        = "espresso-bench-${var.name}"
}

# A DB subnet group needs subnets in two AZs, unlike the hosts, which sit in one.
data "aws_subnets" "vpc" {
  count = local.rds_enabled ? 1 : 0

  filter {
    name   = "vpc-id"
    values = [data.aws_vpc.default.id]
  }
}

resource "aws_db_subnet_group" "this" {
  count = local.rds_enabled ? 1 : 0

  name       = local.rds_name
  subnet_ids = data.aws_subnets.vpc[0].ids
}

# Every parameter is pending-reboot: the instance boots with the group attached, so all of
# them are in effect from the start, and static ones (shared_buffers, huge_pages) cannot use
# any other apply method.
resource "aws_db_parameter_group" "this" {
  count = local.rds_enabled ? 1 : 0

  name   = local.rds_name
  family = "postgres${split(".", var.rds.engine_version)[0]}"

  dynamic "parameter" {
    for_each = var.rds.parameters
    content {
      name         = parameter.key
      value        = parameter.value
      apply_method = "pending-reboot"
    }
  }
}

resource "aws_db_instance" "this" {
  count = local.rds_enabled ? 1 : 0

  identifier     = local.rds_name
  engine         = "postgres"
  engine_version = var.rds.engine_version
  instance_class = var.rds.instance_class

  availability_zone      = var.az
  db_subnet_group_name   = aws_db_subnet_group.this[0].name
  parameter_group_name   = aws_db_parameter_group.this[0].name
  vpc_security_group_ids = [aws_security_group.this.id]
  publicly_accessible    = false

  storage_type       = "gp3"
  allocated_storage  = var.rds.gb
  iops               = var.rds.iops
  storage_throughput = var.rds.mbps

  db_name  = "espresso"
  username = "bench"
  password = var.rds_password

  skip_final_snapshot        = true
  deletion_protection        = false
  backup_retention_period    = 0
  apply_immediately          = true
  auto_minor_version_upgrade = false

  timeouts {
    create = "25m"
    delete = "25m"
  }

  # The schedule is the dead-man switch: it must exist before the instance starts billing. If it
  # fires while the instance is still creating, the scheduler's default retry policy (24 h)
  # repeats the delete until the instance is deletable.
  depends_on = [aws_scheduler_schedule.rds_delete]

  lifecycle {
    precondition {
      condition     = var.rds_password != null && var.reaper_at != null
      error_message = "rds needs rds_password and reaper_at."
    }
  }
}

# The role can delete this one instance and this one file system, nothing else.
resource "aws_iam_role" "scheduler" {
  count = local.reaper_enabled ? 1 : 0

  name = local.rds_name
  path = "/espresso-bench/${var.region}/"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Effect    = "Allow"
      Principal = { Service = "scheduler.amazonaws.com" }
      Action    = "sts:AssumeRole"
      Condition = { StringEquals = { "aws:SourceAccount" = local.account_id } }
    }]
  })
}

# The instance ARN and identifier are built from the name, not read from aws_db_instance: the
# schedule then exists before the instance does and covers its 10-25 minute creation.
resource "aws_iam_role_policy" "scheduler" {
  count = local.rds_enabled ? 1 : 0

  name = "delete-rds"
  role = aws_iam_role.scheduler[0].id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Effect   = "Allow"
      Action   = "rds:DeleteDBInstance"
      Resource = "arn:aws:rds:${var.region}:${local.account_id}:db:${local.rds_name}"
    }]
  })
}

resource "aws_scheduler_schedule_group" "this" {
  count = local.reaper_enabled ? 1 : 0

  name = local.rds_name
}

# `reaper_at` is in UTC, the schedule's default time zone. The provider has no
# action_after_completion, so a fired schedule stays until `tofu destroy` removes it.
resource "aws_scheduler_schedule" "rds_delete" {
  count = local.rds_enabled ? 1 : 0

  name       = "rds-delete"
  group_name = aws_scheduler_schedule_group.this[0].name

  schedule_expression = "at(${var.reaper_at})"

  flexible_time_window {
    mode = "OFF"
  }

  target {
    arn      = "arn:aws:scheduler:::aws-sdk:rds:deleteDBInstance"
    role_arn = aws_iam_role.scheduler[0].arn
    input = jsonencode({
      DbInstanceIdentifier   = local.rds_name
      SkipFinalSnapshot      = true
      DeleteAutomatedBackups = true
    })
  }

  depends_on = [aws_iam_role_policy.scheduler]
}

# Fargate query nodes. Tasks and task definitions are created and deleted by aws-bench; the
# resources here are the static part. A task ends itself at the fleet's expiry (its wrapper runs
# under `timeout`), so only the file system, which bills per GB, needs a schedule.
resource "aws_ecs_cluster" "this" {
  count = local.fargate_enabled ? 1 : 0

  name = local.rds_name

  setting {
    name  = "containerInsights"
    value = "disabled"
  }
}

resource "aws_cloudwatch_log_group" "ecs" {
  count = local.fargate_enabled ? 1 : 0

  name              = "/espresso-bench/${var.name}"
  retention_in_days = 1
}

# ghcr is public, so pulling needs no ECR rights; the tasks have no role of their own.
resource "aws_iam_role" "ecs_exec" {
  count = local.fargate_enabled ? 1 : 0

  name = "${local.rds_name}-exec"
  path = "/espresso-bench/${var.region}/"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Effect    = "Allow"
      Principal = { Service = "ecs-tasks.amazonaws.com" }
      Action    = "sts:AssumeRole"
      Condition = { StringEquals = { "aws:SourceAccount" = local.account_id } }
    }]
  })
}

resource "aws_iam_role_policy" "ecs_exec" {
  count = local.fargate_enabled ? 1 : 0

  name = "write-logs"
  role = aws_iam_role.ecs_exec[0].id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Effect   = "Allow"
      Action   = ["logs:CreateLogStream", "logs:PutLogEvents"]
      Resource = "${aws_cloudwatch_log_group.ecs[0].arn}:*"
    }]
  })
}

# Elastic throughput: the payload writes of a 200 MB/s run exceed any bursting baseline.
resource "aws_efs_file_system" "this" {
  count = local.fargate_enabled ? 1 : 0

  encrypted        = true
  performance_mode = "generalPurpose"
  throughput_mode  = "elastic"

  tags = {
    Name = local.rds_name
  }

  lifecycle {
    precondition {
      condition     = var.reaper_at != null
      error_message = "fargate needs reaper_at."
    }
  }
}

resource "aws_efs_mount_target" "this" {
  count = local.fargate_enabled ? 1 : 0

  file_system_id  = aws_efs_file_system.this[0].id
  subnet_id       = local.pinned_subnet
  security_groups = [aws_security_group.this.id]
}

# One directory per node, so a node sees its own `/efs` and ctl sees all of them at the root.
resource "aws_efs_access_point" "node" {
  for_each = local.fargate_enabled ? toset(var.fargate.nodes) : toset([])

  file_system_id = aws_efs_file_system.this[0].id

  posix_user {
    uid = 0
    gid = 0
  }

  root_directory {
    path = "/${each.key}"
    creation_info {
      owner_uid   = 0
      owner_gid   = 0
      permissions = "0755"
    }
  }
}

resource "aws_iam_role_policy" "scheduler_efs" {
  count = local.fargate_enabled ? 1 : 0

  name = "delete-efs"
  role = aws_iam_role.scheduler[0].id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Effect = "Allow"
      Action = [
        "elasticfilesystem:DeleteMountTarget",
        "elasticfilesystem:DeleteAccessPoint",
        "elasticfilesystem:DeleteFileSystem",
      ]
      Resource = concat(
        [aws_efs_file_system.this[0].arn],
        [for ap in aws_efs_access_point.node : ap.arn],
      )
    }]
  })
}

# The ids are generated, so these schedules exist only once their targets do; the file system
# costs next to nothing in the meantime.
resource "aws_scheduler_schedule" "efs_mount_target_delete" {
  count = local.fargate_enabled ? 1 : 0

  name       = "efs-mount-target-delete"
  group_name = aws_scheduler_schedule_group.this[0].name

  schedule_expression = "at(${var.reaper_at})"

  flexible_time_window {
    mode = "OFF"
  }

  target {
    arn      = "arn:aws:scheduler:::aws-sdk:efs:deleteMountTarget"
    role_arn = aws_iam_role.scheduler[0].arn
    input    = jsonencode({ MountTargetId = aws_efs_mount_target.this[0].id })
  }

  depends_on = [aws_iam_role_policy.scheduler_efs]
}

resource "aws_scheduler_schedule" "efs_access_point_delete" {
  for_each = aws_efs_access_point.node

  name       = "efs-access-point-delete-${each.key}"
  group_name = aws_scheduler_schedule_group.this[0].name

  schedule_expression = "at(${var.reaper_at})"

  flexible_time_window {
    mode = "OFF"
  }

  target {
    arn      = "arn:aws:scheduler:::aws-sdk:efs:deleteAccessPoint"
    role_arn = aws_iam_role.scheduler[0].arn
    input    = jsonencode({ AccessPointId = each.value.id })
  }

  depends_on = [aws_iam_role_policy.scheduler_efs]
}

# DeleteFileSystem fails until the mount target is gone; the scheduler's default retry policy
# (24 h) repeats it.
resource "aws_scheduler_schedule" "efs_delete" {
  count = local.fargate_enabled ? 1 : 0

  name       = "efs-delete"
  group_name = aws_scheduler_schedule_group.this[0].name

  schedule_expression = "at(${formatdate("YYYY-MM-DD'T'hh:mm:ss", timeadd("${var.reaper_at}Z", "15m"))})"

  flexible_time_window {
    mode = "OFF"
  }

  target {
    arn      = "arn:aws:scheduler:::aws-sdk:efs:deleteFileSystem"
    role_arn = aws_iam_role.scheduler[0].arn
    input    = jsonencode({ FileSystemId = aws_efs_file_system.this[0].id })
  }

  depends_on = [aws_iam_role_policy.scheduler_efs]
}
