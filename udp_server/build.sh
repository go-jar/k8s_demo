#!/bin/sh

K8S_REPO="hub.aa.ai/infra/xremote"

cargo build --release

md5sum target/release/test-udp-server > sum.md5
docker build -t "${K8S_REPO}/test-udp-server:1.0.0" -f k8s/Dockerfile .
docker push "${K8S_REPO}/test-udp-server:1.0.0"
rm sum.md5
