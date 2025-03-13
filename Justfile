[private]
@default: help

VERSION := `grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/g'`

# show help message
@help:
    echo "Task runner for Gixor {{ VERSION }} with Just"
    echo "Usage: just <recipe>"
    echo ""
    just --list

formats:
    cargo fmt -- --emit=files

clippy:
    cargo clippy --all-targets --all-features -- -D warnings

build target = "": formats clippy
    cargo build {{ target }}

test: (build "")
    cargo llvm-cov --lcov --output-path target/coverage.lcov

# Generate completion files
completions:
    cargo run -- --generate-completion-files --completion-out-dir assets/completions

prepare_site_build:
    test -d docs/public || git worktree add -f docs/public gh-pages

# Convert the ubild tool definition file (pkl) into the JSON file
pkl2json:
    pkl eval -f json assets/buildtools.pkl > assets/buildtools.json

# Generate the document site with Hugo
site: prepare_site_build
    hugo -s site

# Build the docker image for gixor
docker:
    docker build -t ghcr.io/tamada/btmeister:latest -t ghcr.io/tamada/btmeister:{{VERSION}} .

# Build the docker image for multiple platforms and push them into ghcr.io
docker_buildx:
    docker buildx build --platform linux/arm64/v8,linux/amd64 --output=type=image,push=true -t ghcr.io/tamada/btmeister:latest -t ghcr.io/tamada/btmeister:{{VERSION}} .