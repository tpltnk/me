#!/bin/bash

if [[ $# -eq 0 ]] ; then
  echo 'No environment arguments supplied'
  exit 1
fi

docker build -t yail-img -f docker/Dockerfile.$1 .

docker run -p 8000:8000 -ti yail-img:latest
