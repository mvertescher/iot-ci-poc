#!/bin/bash
#
# Convert a `pi-gen` image into a Mender compatible image.

set -eux

cd ./mender-convert

# Build them mender-convert Docker image
./docker-build

# Retrieve the input image
IMG=$(ls -t ../pi-gen/deploy/*runner.img | head -1)
mkdir -p input && cp $IMG input/runner.img

# Convert the image to a format that can be deployed via mender
MENDER_ARTIFACT_NAME=release-1 ./docker-mender-convert \
  --disk-image input/runner.img \
  --config ./configs/raspberrypi4_config \
  --overlay ./rootfs_overlay_demo/

ls -hal deploy
