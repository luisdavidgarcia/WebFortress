#!/bin/bash

set -e

mkdir -p mountdir
cargo build
cp /bin/bash ./mountdir/