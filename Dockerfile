FROM rust:1.89-alpine3.22 AS build

ARG pkg=midi-mp3-api

WORKDIR /build

COPY . .

RUN apk add --no-cache build-base binutils

RUN --mount=type=cache,target=/build/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    set -eux; \
    cargo build --release; \
    objcopy --compress-debug-sections target/release/$pkg ./waveemapi3

FROM alpine:3.22

WORKDIR /et

COPY --from=build /build/waveemapi3 ./
RUN mkdir ./data

ENV WAVEEMAPI_ADDRESS=0.0.0.0
ENV WAVEEMAPI_PORT=8080
ENV WAVEEMAPI_DATA_PATH=./data
ENV WAVEEMAPI_LIMITS={form="1 GiB",data-form="1 GB",file="500 MB"}

CMD ["./waveemapi3"]