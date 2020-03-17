#!/bin/bash
#
# Flash a raw `pi-gen` image to an sdcard (mmcblk0 by default).

set -eu

: "${DEVICE:=/dev/mmcblk0}"

IMG=$(ls ./pi-gen/deploy/*runner.img | head -1)

echo "Flashing " $IMG " ..."
sudo dd bs=4M if=$IMG of=$DEVICE conv=fsync
echo "Done!"
