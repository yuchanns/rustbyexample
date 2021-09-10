#!/usr/bin/env bash

set -eu

val=0

while [[ $val -lt 10 ]]; do
    echo $val
    val=$(($val + 1))
    sleep 1
done