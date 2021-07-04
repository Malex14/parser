FROM docker.io/library/alpine:edge as builder
WORKDIR /build
RUN apk --no-cache upgrade \
    && apk --no-cache add cargo \
    && rustc --version && cargo --version

# cargo needs a dummy src/main.rs to detect bin mode
RUN mkdir -p src && echo "fn main() {}" > src/main.rs

COPY Cargo.toml Cargo.lock ./
RUN cargo build --release --locked

# We need to touch our real main.rs file or the cached one will be used.
COPY . ./
RUN touch src/main.rs

RUN cargo build --release --locked


# Start building the final image
FROM docker.io/library/alpine
VOLUME /app/calendars
VOLUME /app/eventfiles
VOLUME /app/userconfig
WORKDIR /app

RUN apk --no-cache upgrade && apk --no-cache add libgcc

COPY --from=builder /build/target/release/hawhh-calendarbot-parser /usr/bin/

ENTRYPOINT ["hawhh-calendarbot-parser"]
