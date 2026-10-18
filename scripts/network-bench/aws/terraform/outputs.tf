output "hosts" {
  value = {
    for k, i in aws_instance.host : k => {
      role        = var.hosts[k].role
      public_ip   = i.public_ip
      private_ip  = i.private_ip
      private_dns = i.private_dns
      instance_id = i.id
    }
  }
}

output "pg_volume_id" {
  description = "Volume id of the query host's Postgres volume; null without one."
  value = one(flatten([
    for k, i in aws_instance.host : [for d in i.ebs_block_device : d.volume_id]
    if var.hosts[k].role == "query"
  ]))
}

output "rds" {
  value = length(aws_db_instance.this) == 0 ? null : {
    identifier     = aws_db_instance.this[0].identifier
    endpoint       = aws_db_instance.this[0].address
    port           = aws_db_instance.this[0].port
    engine_version = aws_db_instance.this[0].engine_version_actual
  }
}

output "fargate" {
  value = !local.fargate_enabled ? null : {
    cluster_arn       = aws_ecs_cluster.this[0].arn
    exec_role_arn     = aws_iam_role.ecs_exec[0].arn
    log_group         = aws_cloudwatch_log_group.ecs[0].name
    subnet_id         = local.pinned_subnet
    security_group_id = aws_security_group.this.id
    efs_id            = aws_efs_file_system.this[0].id
    efs_dns           = aws_efs_file_system.this[0].dns_name
    access_points     = { for node, ap in aws_efs_access_point.node : node => ap.id }
  }

  # The mount target makes the file system reachable; ctl mounts it right after `apply`.
  depends_on = [aws_efs_mount_target.this]
}
