FROM ubuntu:14.04

RUN apt-get update \
 && apt-get install -y --no-install-recommends \
        build-essential \
        ca-certificates \
        clang \
        cppcheck \
        curl \
        gdb \
        strace \
        valgrind \
 && rm -rf /var/lib/apt/lists/* \
 && apt-get clean

ENV CHECK_VER=0.10.0
RUN curl -sSfL https://github.com/libcheck/check/releases/download/${CHECK_VER}/check-${CHECK_VER}.tar.gz \
  | tar zxvf - -C /usr/local/src \
 && cd /usr/local/src/check-${CHECK_VER} \
 && ./configure --prefix=/usr --disable-timeout-tests \
 && make \
 && echo "Prepare for a several minute timeout test...." \
 && make check \
 && make install
