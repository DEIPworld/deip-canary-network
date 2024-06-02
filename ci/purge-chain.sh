#!/bin/bash

docker-compose -f docker-compose2.yml -f multi-node.yml run --no-deps node1 appchain-deip purge-chain --base-path /var/lib/deipd --chain dev
docker-compose -f docker-compose2.yml -f multi-node.yml run --no-deps node2 appchain-deip purge-chain --base-path /var/lib/deipd --chain dev
docker-compose -f docker-compose2.yml -f multi-node.yml run --no-deps node3 appchain-deip purge-chain --base-path /var/lib/deipd --chain dev
docker-compose -f docker-compose2.yml -f multi-node.yml run --no-deps node4 appchain-deip purge-chain --base-path /var/lib/deipd --chain dev
docker-compose -f docker-compose2.yml -f multi-node.yml run --no-deps node5 appchain-deip purge-chain --base-path /var/lib/deipd --chain dev
docker-compose -f docker-compose2.yml -f multi-node.yml run --no-deps node6 appchain-deip purge-chain --base-path /var/lib/deipd --chain dev
