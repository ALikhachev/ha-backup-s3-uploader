#!/bin/sh

set -e

echo 'Building arm64 variant'

cross build --target aarch64-unknown-linux-musl --release

ln -s aarch64-unknown-linux-musl target/arm64-unknown-linux-musl

echo 'Building x86-64 variant'

cross build --target x86_64-unknown-linux-musl --release

ln -s x86_64-unknown-linux-musl target/amd64-unknown-linux-musl

echo 'Creating images'

docker buildx build --platform linux/amd64,linux/arm64 --tag alexlikhachev/ha-backup-uploader:latest .

echo 'Build successfully finished'
