FROM rust:1.67.1

RUN rustup target add wasm32-wasi
WORKDIR /usr/src
ADD rust-benchmark rust-benchmark
WORKDIR /usr/src/rust-benchmark
RUN cargo build --release --target=wasm32-wasi
RUN cp target/wasm32-wasi/release/bls12-377-pairing.wasm /benchmark.wasm