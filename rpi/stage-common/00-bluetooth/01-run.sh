#!/bin/bash -e

# Add the default user to the bluetooth group
on_chroot << EOF
usermod -a -G bluetooth $FIRST_USER_NAME
EOF
