FROM circleci/openjdk:9

USER root

RUN apt-get update
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly
RUN (curl -sL https://deb.nodesource.com/setup_9.x | bash -) && apt-get install -y nodejs
