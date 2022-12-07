#!/bin/bash

die() {
    local message=$1
    echo "$message" >&2
    exit 1
}
exec 8<>/dev/udp/127.0.0.1/34254 || die "Failed to open Udp port"
echo "Hi">&8 || die "Failed to connect Udp server"

cat<&8 || die "Failed to receive by Udp"
