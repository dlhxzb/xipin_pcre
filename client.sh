#!/bin/bash

exec 8<>/dev/udp/127.0.0.1/34254

while true; do 
    echo "">&8
done