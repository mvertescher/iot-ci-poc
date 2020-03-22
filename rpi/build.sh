#!/bin/bash
#
# Custom Raspberry Pi image creation.
#
# This script overlays custom configuration and stages to `pi-gen` and builds
# in Docker.

set -eux

cd pi-gen

# Reset the submodule
git checkout . && git clean -dxf

# Add custom stage
cp -r ../stage-common .

echo "IMG_NAME='runner'
DEPLOY_ZIP=0
USE_QEMU=0
LOCALE_DEFAULT=en_US.UTF-8
TARGET_HOSTNAME=rpi
KEYBOARD_KEYMAP=gb
KEYBOARD_LAYOUT='English (UK)'
TIMEZONE_DEFAULT='Europe/London'
FIRST_USER_NAME=pi
FIRST_USER_PASS=raspberry
ENABLE_SSH=1
STAGE_LIST='stage0 stage1 stage2 stage-common'
" > config

# Don't create images for the 'lite' build
touch ./stage2/SKIP_IMAGES

# Actually run pi-gen
# TODO: Add clean option: docker rm -v pigen_work && ./build-docker.sh
PRESERVE_CONTAINER=1 CONTINUE=1 ./build-docker.sh

# Clean up
rm -rf stage-common
