FROM rust:1.60-buster
RUN apt-get update && apt-get -y install \
  libssl-dev \
  pkg-config \
  procps \
  curl \
  postgresql-client-11 \
  inotify-tools \
&& apt-get clean \
&& rm -rf /var/lib/apt/lists/*
WORKDIR /study/study
