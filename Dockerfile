FROM alpine:3.21

ARG TARGETARCH

WORKDIR /app

COPY target/$TARGETARCH-unknown-linux-musl/release/ha-backup-uploader /app

CMD ["./ha-backup-uploader", "/backups"]