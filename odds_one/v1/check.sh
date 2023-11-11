#!/bin/bash

set -e

FILE="samples.json"

START=$(date +%s%N)
TEST=$(eval "$1 $FILE")
END=$(date +%s%N)
ELAPSED=$((($END - $START) / 1000))
REF=$(eval "./rust_bucket $FILE")

if [ -z "${TEST##*$REF*}" ]; then
  echo "Test cases passed in ${ELAPSED}us."
else
  echo "Test cases failed."
fi



