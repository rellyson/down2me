FROM rust:1.50.0-slim

WORKDIR /src/app

RUN apt update && apt upgrade -y
RUN apt install -y \
    git \
    gcc \
    nano \
    openssl \
    libssl-dev \
    libssh-dev \
    cmake \
    pkg-config \
    openssl  \
    curl \
    wget \
    python3 \
    python3-pip

RUN curl http://archive.ubuntu.com/ubuntu/pool/universe/y/youtube-dl/youtube-dl_2021.02.10-1_all.deb  \
    --output youtube-dl.deb

RUN curl http://archive.ubuntu.com/ubuntu/pool/main/p/python-setuptools/python3-pkg-resources_39.0.1-2_all.deb  \
    --output python3-pkg-resources.deb

RUN dpkg -i python3-pkg-resources.deb
RUN dpkg -i youtube-dl.deb
    
RUN rustup default nightly

COPY . .

CMD [ "cargo", "run" ]