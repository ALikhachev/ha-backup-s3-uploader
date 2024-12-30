# Home Assistant Backup S3 Uploader [![Docker Image Version](https://img.shields.io/docker/v/alexlikhachev/ha-backup-s3-uploader)](https://hub.docker.com/r/alexlikhachev/ha-backup-s3-uploader) ![GitHub License](https://img.shields.io/github/license/ALikhachev/ha-backup-s3-uploader)

A simple tool for uploading Home Assistant backups to S3 or S3-compatible storage (like Minio). Built as an experiment
to explore the Rust ecosystem!

## Features

* Works with S3 or S3-compatible storage such as Minio.
* Uploads backups with their actual names, e.g., replacing `99733641.tar` with `test-backup.tar` (if that was the
  original name of the backup).

## How to use

1. Get your storage credentials and configure permissions.
2. Run the Docker image `alexlikhachev/ha-backup-s3-uploader`, mounting the Home Assistant backups directory to
   `/backups`, as follows:
   ```shell
   docker run --rm \
   -e AWS_ACCESS_KEY_ID='<access-key>' \
   -e AWS_SECRET_ACCESS_KEY='<secret-key>' \
   -e AWS_SESSION_TOKEN='[session-token]' \
   -e AWS_REGION='<region>' \
   -e AWS_ENDPOINT='[custom-endpoint]' \
   -e AWS_BUCKET='<bucket-name>' \
   -v <backup-directory>:/backups \
   alexlikhachev/ha-backup-s3-uploader:latest
   ```
   The `<backup-directory>` could be something like `/usr/share/hassio/backup`.
3. The application will upload any existing backups and terminate once completed.
4. To upload backups regularly, configure it via crontab.

## TODO

* Better integration with Home Assistant.
* Improved error handling.
* ...

## How to build (tested only on macOS so far)

1. Install the Rust toolchain: [Install Rust](https://www.rust-lang.org/tools/install).
2. Install Cross for cross-compilation:
   ```bash
   cargo install cross
   ```
3. Build the project using the provided script:
   ```bash
   ./build.sh
   ```