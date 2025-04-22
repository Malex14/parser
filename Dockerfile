FROM docker.io/library/rust:1-bookworm AS builder
WORKDIR /build
RUN apt-get update \
	&& apt-get upgrade -y \
	&& apt-get clean \
	&& rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./

# cargo needs a dummy src/lib.rs to compile the dependencies
RUN mkdir -p src \
	&& touch src/lib.rs \
	&& cargo build --release --locked \
	&& rm -rf src

COPY . ./
RUN cargo build --release --locked --offline


FROM docker.io/library/debian:bookworm-slim AS final
RUN apt-get update \
	&& apt-get upgrade -y \
	&& apt-get clean \
	&& rm -rf /var/lib/apt/lists/* /var/cache/* /var/log/*

WORKDIR /app
VOLUME /app/calendars
VOLUME /app/eventfiles
VOLUME /app/userconfig

COPY --from=builder /build/target/release/hawhh-calendarbot-parser /usr/local/bin/
ENTRYPOINT ["hawhh-calendarbot-parser"]
