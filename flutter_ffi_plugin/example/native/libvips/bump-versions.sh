#!/usr/bin/env bash
#
set -eu
cur_ver=$(grep "ENV VIPS_VERSION" generator/Dockerfile | awk -F'=' {'print $2'})
new_ver="${1}"

sed -i "s/$cur_ver/$new_ver/" README.md
