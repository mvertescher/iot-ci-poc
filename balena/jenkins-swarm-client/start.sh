#!/bin/sh
#
# Jenkins swarm client start script.
#

SERIAL=$(cat /proc/cpuinfo | grep Serial | cut -d ' ' -f 2)

java -jar ./swarm-client.jar \
  -master http://jenkins.local:8080 \
  -name swarm-$SERIAL \
  -username swarm \
  -password swarm \
  -description "I'm a Raspberry Pi!" \
  -executors 1 \
  -mode exclusive \
  -disableClientsUniqueId \
  -retryInterval 60
