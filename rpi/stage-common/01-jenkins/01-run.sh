#!/bin/bash -e

# Create a Jenkins user
on_chroot << EOF
if ! id -u jenkins >/dev/null 2>&1; then
  useradd -d /home/jenkins jenkins
fi
EOF

# Create home directory
mkdir -p "${ROOTFS_DIR}/home/jenkins"

# Download and install the swarm client jar file
curl -o files/swarm-client.jar \
  https://repo.jenkins-ci.org/releases/org/jenkins-ci/plugins/swarm-client/3.18/swarm-client-3.18.jar
install -m 644 files/swarm-client.jar "${ROOTFS_DIR}/home/jenkins/"

# Install the swarm-client start script and systemd unit file
install -m 774 files/start.sh "${ROOTFS_DIR}/home/jenkins/"
install -m 644 files/jenkins-swarm-client.service \
  "${ROOTFS_DIR}/usr/lib/systemd/system"

# Ensure the `labels` file exists
install -m 777 files/labels "${ROOTFS_DIR}/home/jenkins/"

# Ensure home directory ownership
on_chroot << EOF
chown -R jenkins:jenkins /home/jenkins
EOF
