# Build Stage
FROM --platform=linux/amd64 rustlang/rust:nightly as builder

ENV DEBIAN_FRONTEND=noninteractive
## Install build dependencies.
RUN apt-get update 
RUN apt-get install -y cmake clang
RUN cargo install cargo-fuzz

## Add source code to the build stage.
ADD . /formats/

WORKDIR /formats/fuzz/

RUN cargo +nightly fuzz build

FROM --platform=linux/amd64 rustlang/rust:nightly

## TODO: Change <Path in Builder Stage>

COPY --from=builder /formats/fuzz/target/x86_64-unknown-linux-gnu/release/base16ct /

COPY --from=builder /formats/fuzz/target/x86_64-unknown-linux-gnu/release/base16ct_higher /

COPY --from=builder /formats/fuzz/target/x86_64-unknown-linux-gnu/release/base16ct_mixed /

COPY --from=builder /formats/fuzz/target/x86_64-unknown-linux-gnu/release/base32ct /

COPY --from=builder /formats/fuzz/target/x86_64-unknown-linux-gnu/release/base32ct_unpadded /

COPY --from=builder /formats/fuzz/target/x86_64-unknown-linux-gnu/release/base32ct_mixed /