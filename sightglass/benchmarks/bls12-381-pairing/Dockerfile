FROM rust:1.66

RUN rustup target add wasm32-wasi
WORKDIR /usr/src
ADD rust-benchmark rust-benchmark
WORKDIR /usr/src/rust-benchmark
RUN rustup install nightly
RUN cargo +nightly build --release --target=wasm32-wasi
RUN ls -la target/wasm32-wasi/release/
RUN cp target/wasm32-wasi/release/bls12-381-pairing.wasm /benchmark.wasm