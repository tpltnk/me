#!/bin/bash

if [[ $# -eq 0 ]] ; then
  echo 'No environment arguments supplied'
  exit 1
fi

docker build -t registry.digitalocean.com/tpltnk-registry/me-img:latest -f docker/Dockerfile.$1 .

docker run -p 8000:8000 -ti registry.digitalocean.com/tpltnk-registry/me-img:latest
