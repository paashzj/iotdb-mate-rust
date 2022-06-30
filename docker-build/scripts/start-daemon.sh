#!/bin/bash

mkdir -p $IOTDB_HOME/logs
nohup $IOTDB_HOME/mate/iotdb_mate >>$IOTDB_HOME/logs/iotdb_mate.stdout.log 2>>$IOTDB_HOME/logs/iotdb_mate.stderr.log &
