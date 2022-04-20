#!/bin/sh

K8S_REPO="hub.aa.ai/infra/xremote"

cargo build --release

docker build -t "${K8S_REPO}/test-udp-server:1.0.0" -f k8s/Dockerfile .
docker push "${K8S_REPO}/test-udp-server:1.0.0"
