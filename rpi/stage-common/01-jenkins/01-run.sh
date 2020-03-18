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
mkdir -p files
curl -o files/swarm-client.jar \
  https://repo.jenkins-ci.org/releases/org/jenkins-ci/plugins/swarm-client/3.18/swarm-client-3.18.jar
# TODO: chown, maybe -g jenkins -o jenkins
install -m 644 files/swarm-client.jar "${ROOTFS_DIR}/home/jenkins/"
