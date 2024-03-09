#!/bin/sh
set -e

# Source: nodejs/docker-entrypoint.sh
if [ "${1#-}" != "${1}" ] || [ -z "$(command -v "${1}")" ] || { [ -f "${1}" ] && ! [ -x "${1}" ]; }; then
  set -- datagen "$@"
fi

exec "$@"
