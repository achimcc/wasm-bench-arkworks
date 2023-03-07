FROM rust:1.66

RUN rustup target add wasm32-wasi
WORKDIR /usr/src
ADD rust-benchmark rust-benchmark
WORKDIR /usr/src/rust-benchmark
RUN rustup install nightly
RUN cargo +nightly build --release --target=wasm32-wasi
RUN cp target/wasm32-wasi/release/ed-on-bls12-381-msm-te-1000.wasm /benchmark.wasm