FROM buildpack-deps:jammy

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

# Install non-rust dependencies
RUN apt update; \
    apt install cmake protobuf-compiler -y; \
    apt-get clean; \
    rm -rf /var/lib/apt/lists/*;

RUN set -eux; \
    dpkgArch="$(dpkg --print-architecture)"; \
    case "${dpkgArch##*-}" in \
    amd64) rustArch='x86_64-unknown-linux-gnu' ;; \
    arm64) rustArch='aarch64-unknown-linux-gnu' ;; \
    *) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;; \
    esac; \
    \
    url="https://static.rust-lang.org/rustup/dist/${rustArch}/rustup-init"; \
    wget "$url"; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --default-toolchain stable; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;

# Slow
#RUN cargo install cargo-audit
# Install binary from github
RUN set -eux; \
    dpkgArch="$(dpkg --print-architecture)"; \
    case "${dpkgArch##*-}" in \
    amd64) rustArch='x86_64-unknown-linux-musl' ;; \
    arm64) rustArch='aarch64-unknown-linux-gnu' ;; \
    *) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;; \
    esac; \
    \
    url="https://github.com/rustsec/rustsec/releases/download/cargo-audit%2Fv0.20.0/cargo-audit-${rustArch}-v0.20.0.tgz"; \
    wget "$url"; \
    tar -xvf cargo-audit-*.tgz --no-anchored cargo-audit ; \
    mv cargo-audit-*/cargo-audit /usr/bin ; \
    rm -r cargo-audit-* ;
RUN cargo-audit -h;

# Slow
#RUN cargo install wasm-pack
# Install binary from github
RUN set -eux; \
    dpkgArch="$(dpkg --print-architecture)"; \
    case "${dpkgArch##*-}" in \
    amd64) rustArch='x86_64-unknown-linux-musl' ;; \
    arm64) rustArch='aarch64-unknown-linux-musl' ;; \
    *) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;; \
    esac; \
    \
    url="https://github.com/rustwasm/wasm-pack/releases/download/v0.13.0/wasm-pack-v0.13.0-${rustArch}.tar.gz"; \
    wget "$url"; \
    tar -xvf wasm-pack-*.tar.gz --no-anchored wasm-pack ; \
    mv wasm-pack-*/wasm-pack /usr/bin ; \
    rm -r wasm-pack-* ;
RUN wasm-pack --version;

# Slow
#RUN cargo install just
# Install binary from github
RUN set -eux; \
    dpkgArch="$(dpkg --print-architecture)"; \
    case "${dpkgArch##*-}" in \
    amd64) rustArch='x86_64-unknown-linux-musl' ;; \
    arm64) rustArch='aarch64-unknown-linux-musl' ;; \
    *) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;; \
    esac; \
    \
    url="https://github.com/casey/just/releases/download/1.35.0/just-1.35.0-${rustArch}.tar.gz"; \
    wget "$url"; \
    tar -xvf just-*.tar.gz just ; \
    mv just /usr/bin ;
RUN just --version;