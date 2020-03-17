# Raspberry Pi CI Runner OS

This directory uses the official `pi-gen` repository to configure a custom
Raspbian Lite based OS to become a CI runner.

> __WARNING__: Use caution when running scripts in this directory, especially
when flashing!

## Overview

`stage-commmon`: Layers that apply to all Raspberry Pi boards. However, this
stage is primarily designed for and tested on the Raspberry Pi 4.

`stage-rpi0w`: Layers specific to the Raspberry Pi Zero W. At the moment, this
stage just enables the device to act as an ethernet gadget.
