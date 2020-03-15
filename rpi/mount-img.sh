#!/bin/bash
#
# Mount a rpi image locally using `kpartx`.
# Adopted from:
#   https://github.com/mozilla-iot/wiki/wiki/Loop-mounting-a-Raspberry-Pi-image-file-under-linux

set -eux

if [ "$EUID" -ne 0 ]; then
  echo "Please run as root"
  exit
fi

IMG=$(ls ./pi-gen/deploy/*talos.img | head -1)

# Super naive mount/unmount toggle
if [ ! -d 'rootfs' ]; then
  kpartx -v -a $IMG
  mkdir -p rootfs
  mount /dev/mapper/loop0p2 rootfs
  mount /dev/mapper/loop0p1 rootfs/boot
else
  umount rootfs/boot
  umount rootfs
  kpartx -v -d $IMG
  rmdir rootfs
fi
