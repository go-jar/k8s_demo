FROM ubuntu:20.04
LABEL maintainer="jingpingyin"

WORKDIR /opt/udp_server
RUN apt update -y && \
    apt install -y --no-install-recommends \
    curl wget sudo jq vim less

COPY sum.md5 /opt/udp_server/sum.md5
COPY target/release/udp_server /opt/udp_server/udp_server
CMD [ "/bin/bash" ]
