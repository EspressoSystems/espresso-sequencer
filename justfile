# The zkevm-node docker-compose file currently only works if run from the zkevm-node/test directory.
compose := "cd zkevm-node/test && docker compose -f permissionless-docker-compose.yml -f ../../docker-compose.yaml --env-file ../../.env"
compose-zkevm-node := "cd zkevm-node/test && docker compose -f permissionless-docker-compose.yml --env-file ../../.env"

zkevm-node:
    {{compose-zkevm-node}} up -V --force-recreate --abort-on-container-exit

zkevm-node-background:
    {{compose-zkevm-node}} up -d -V --force-recreate

demo:
    {{compose}} up -V --force-recreate --abort-on-container-exit

demo-background:
    {{compose}} up -d -V --force-recreate

down:
    {{compose}} down -v --remove-orphans

pull:
    {{compose}} pull

hardhat *args:
    cd zkevm-contracts && nix develop -c bash -c "npx hardhat {{args}}"

npm *args:
   cd zkevm-contracts && nix develop -c bash -c "npm {{args}}"

compose *args:
   {{compose}} {{args}}

docker-stop-rm:
    docker stop $(docker ps -aq); docker rm $(docker ps -aq)
