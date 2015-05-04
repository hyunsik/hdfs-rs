#!/bin/bash

if [ -z $HADOOP_HOME ]; then
  echo "HADOOP_HOME is not set."
  exit -1
fi
export LD_LIBRARY_PATH=$HADOOP_HOME/lib/native

HADOOP_MODULE_DIRS="$HADOOP_HOME/share/hadoop/common/lib/
$HADOOP_HOME/share/hadoop/common/
$HADOOP_HOME/share/hadoop/hdfs
$HADOOP_HOME/share/hadoop/hdfs/lib/
$HADOOP_HOME/share/hadoop/yarn/lib/
$HADOOP_HOME/share/hadoop/yarn/"

HADOOP_CONF_DIR=$HADOOP_HOME/etc/hadoop
CLASSPATH="${HADOOP_CONF_DIR}"

for d in $HADOOP_MODULE_DIRS; do
  for j in $d/*.jar; do
    CLASSPATH=${CLASSPATH}:${j}
  done;
done;

export CLASSPATH
export LD_LIBRARY_PATH=${LD_LIBRARY_PATH}:${JAVA_HOME}/jre/lib/server

$@
