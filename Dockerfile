FROM rust:1.85-alpine AS builder

RUN apk add --no-cache \
    curl \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    pkgconfig

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir -p najm-backend/src najm-cms/src najm-dimentorin/src \
    najm-entity/src najm-gacha/src najm-gateway/src \
    najm-iam/src najm-lib/src najm-middleware/src \
    najm-util/src tests/src && \
    echo "fn main() {}" > najm-backend/src/main.rs && \
    find . -name "src" -type d -exec sh -c 'echo "// dummy" > "$1/lib.rs"' _ {} \;

RUN echo '[package]\nname = "tests"\nversion = "0.1.0"\nedition = "2021"' > tests/Cargo.toml


RUN echo -e '[package]\nname = "tests"\nversion = "0.1.0"\nedition = "2021"' > tests/Cargo.toml


COPY najm-backend ./najm-backend
COPY najm-cms ./najm-cms
COPY najm-dimentorin ./najm-dimentorin
COPY najm-entity ./najm-entity
COPY najm-gacha ./najm-gacha
COPY najm-gateway ./najm-gateway
COPY najm-iam ./najm-iam
COPY najm-lib ./najm-lib
COPY najm-middleware ./najm-middleware
COPY najm-util ./najm-util
COPY tests ./tests

RUN RUSTFLAGS="-C target-cpu=generic -C opt-level=s -C panic=abort -C codegen-units=1 -C strip=symbols" \
    cargo build -p najm-backend --release && \
    strip target/release/api && \
    upx --best --lzma target/release/api 2>/dev/null || true

FROM scratch AS runner
COPY --from=builder /app/target/release/api /api
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
ENTRYPOINT ["/api"]