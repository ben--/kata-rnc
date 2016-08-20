FROM ubuntu:14.04

RUN apt-get update \
 && apt-get install -y --no-install-recommends \
        build-essential \
        gdb \
        strace \
 && rm -rf /var/lib/apt/lists/* \
 && apt-get clean
