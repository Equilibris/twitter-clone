# from https://kerkour.com/deploy-rust-on-heroku-with-docker
####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

RUN rustup toolchain install nightly
RUN rustup target add --toolchain nightly x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# Create appuser
ENV USER=backend
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /backend

COPY ./backend/ .

RUN cargo +nightly build --target x86_64-unknown-linux-musl --release

####################################################################################################
## Final image
####################################################################################################
FROM alpine:latest

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /backend

# Copy our build
COPY --from=builder /backend/target/x86_64-unknown-linux-musl/release/backend ./

# Use an unprivileged user.
USER backend:backend

CMD ["/backend/backend"]
