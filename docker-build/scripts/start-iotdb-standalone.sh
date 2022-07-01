#!/bin/bash

mkdir -p $IOTDB_HOME/logs
nohup $IOTDB_HOME/sbin/start-server.sh >>$IOTDB_HOME/logs/iotdb.stdout.log 2>>$IOTDB_HOME/logs/iotdb.stderr.log &
