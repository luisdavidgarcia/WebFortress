#!/bin/bash

set -e

mkdir -p mountdir
cp /bin/bash ./mountdir/
cargo build
