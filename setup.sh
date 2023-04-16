#!/bin/bash

MAELSTROM_PATH=$(pwd)/sbin/maelstrom
MAELSTROM_BIN=${MAELSTROM_PATH}/maelstrom

if [[ ! -f $MAELSTROM_BIN ]]; then 
  echo "error maelstrom not found at path: $MAELSTROM_BIN"
  exit 1
fi

export PATH=${PATH}:$MAELSTROM_PATH
