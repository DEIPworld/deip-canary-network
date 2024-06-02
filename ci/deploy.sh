#!/bin/bash

set -e

export BUILD_SOURCE=${BUILD_SOURCE?Please specify BUILD_SOURCE directory}
export BUILD_CACHE=${BUILD_CACHE?Please specify BUILD_CACHE directory}

export REVISION=$(git -C $BUILD_SOURCE rev-parse --short HEAD)

cat /dev/null > .env

LOCAL_REPO="localhost"
echo "LOCAL_REPO=$LOCAL_REPO" >> .env

export BUILD_SCRIPT="event-proxy/build.sh"
export BUILD_WORKDIR="event-proxy"
export BINARY_NAME="event-proxy"
export APP_IMAGE="$BINARY_NAME:$REVISION"
./build_img.sh && docker tag $APP_IMAGE "$LOCAL_REPO/$APP_IMAGE"
echo "EVENT_PROXY_IMG=$APP_IMAGE" >> .env
echo "EVENT_PROXY_BIN=$BINARY_NAME" >> .env

export BUILD_SCRIPT="node/build.sh"
export BUILD_WORKDIR="."
export BINARY_NAME="appchain-deip"
export APP_IMAGE="$BINARY_NAME:$REVISION"
./build_img.sh && docker tag $APP_IMAGE "$LOCAL_REPO/$APP_IMAGE"
echo "BLOCKCHAIN_NODE_IMG=$APP_IMAGE" >> .env
echo "BLOCKCHAIN_NODE_BIN=$BINARY_NAME" >> .env

export BUILD_SCRIPT="event-proxy-client/build.sh"
export BUILD_WORKDIR="event-proxy-client"
export BINARY_NAME="event-proxy-client"
export APP_IMAGE="$BINARY_NAME:$REVISION"
./build_img.sh && docker tag $APP_IMAGE "$LOCAL_REPO/$APP_IMAGE"
echo "EVENT_PROXY_CLIENT_IMG=$APP_IMAGE" >> .env
echo "EVENT_PROXY_CLIENT_BIN=$BINARY_NAME" >> .env

echo "COMMON_NODE_PARAMS=\
--chain=/spec.json \
--base-path=/var/lib/deipd \
--node-key-file=/var/lib/deipd/node-key-file \
--ws-external --rpc-external --rpc-methods=Unsafe \
--offchain-worker=WhenValidating --enable-offchain-indexing=true \
-lpallet_deip_proposal=debug,pallet_deip_dao=debug,pallet_deip_portal=debug \
-l=txpool=trace,runtime=trace,offchain=trace" >> .env

docker-compose up -d --no-build


# appchain-deip build-spec --chain=/spec/devSpec.json --raw --disable-default-bootnode > /spec/devSpecRaw.json
# docker run --init --rm -ti -v $(pwd):/spec appchain-deip:478b54a bash
# appchain-deip purge-chain --base-path=/var/lib/deipd --chain=dev -y
