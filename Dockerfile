####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# Create appuser
ENV USER=service-utils
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /service-utils

COPY src /service-utils/src
COPY Cargo.toml /service-utils/Cargo.toml
COPY Cargo.lock /service-utils/Cargo.lock

RUN cargo build --target x86_64-unknown-linux-musl --release

####################################################################################################
## Final image
####################################################################################################
FROM alpine

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /service-utils

# Copy our build
COPY --from=builder /service-utils/target/x86_64-unknown-linux-musl/release/service-utils ./

# Use an unprivileged user.
#USER service-utils:service-utils

CMD ["/service-utils/service-utils"]
