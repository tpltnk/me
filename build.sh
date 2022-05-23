#!/bin/bash

docker build -t yail-img -f docker/Dockerfile.debug .

docker run -p 8000:8000 -ti yail-img:latest
