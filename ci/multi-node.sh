#!/bin/bash

docker-compose -f docker-compose2.yml -f multi-node.yml "$@"
