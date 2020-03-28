#!/bin/sh
#
# Jenkins swarm client start script.
#

: "${JENKINS_MASTER_URL:=http://jenkins.local:8080}"
: "${JENKINS_USERNAME:=swarm}"
: "${JENKINS_PASSWORD:=swarm}"

SERIAL=$(cat /proc/cpuinfo | grep Serial | cut -d ' ' -f 2)

java -jar ./swarm-client.jar \
  -master $JENKINS_MASTER_URL \
  -name swarm-$SERIAL \
  -username $JENKINS_USERNAME \
  -password $JENKINS_PASSWORD \
  -description "I'm a Raspberry Pi!" \
  -executors 1 \
  -mode exclusive \
  -disableClientsUniqueId \
  -retryInterval 60
