FROM rust:latest

WORKDIR /tmp

RUN apt-get update

RUN apt-get install curl
RUN curl -fsSL https://deb.nodesource.com/setup_19.x | bash -
RUN apt-get install -y nodejs

RUN apt-get install -y cmake libssl-dev zlib1g-dev gcc g++ clang
RUN apt-get install -y default-mysql-client

RUN git clone https://github.com/rui314/mold.git \
    && mkdir mold/build \
    && cd mold/build \
    && git checkout v1.6.0 \
    && ../install-build-deps.sh \
    && cmake -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_COMPILER=c++ .. \
    && cmake --build . -j $(nproc) \
    && cmake --install .

RUN rustup component add rustfmt
RUN cargo install cargo-watch

WORKDIR /app
# CMD cargo watch -s 'mold -run cargo run'
