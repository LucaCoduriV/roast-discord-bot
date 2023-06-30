FROM rust:buster AS builder

WORKDIR /prod
COPY Cargo.lock .
COPY Cargo.toml .
COPY .env .
RUN mkdir .cargo
# This is the trick to speed up the building process.
RUN cargo vendor > .cargo/config

COPY . .
COPY .env .
RUN cargo build --release

# Use any runner as you want
# But beware that some images have old glibc which makes rust unhappy
FROM fedora:34 AS runner
COPY --from=builder /prod/target/release/roast-discord-bot /home
COPY .env /home
COPY .env /
# Run the app
CMD /home/roast-discord-bot