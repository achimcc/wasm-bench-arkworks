FROM rust:1.66

RUN rustup target add wasm32-wasi
WORKDIR /usr/src
ADD rust-benchmark rust-benchmark
WORKDIR /usr/src/rust-benchmark
RUN cargo build --release --target=wasm32-wasi
RUN cp target/wasm32-wasi/release/bls12-377-mul-projective-g1.wasm /benchmark.wasm