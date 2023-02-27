FROM rust:1.67

WORKDIR /src
# COPY Cargo.toml Cargo.lock ./
# RUN cargo install --path .
COPY . .
RUN cargo build --release --target-dir=./target/release

CMD [ "./target/release/release/sorting-algo-func-rs" ]
