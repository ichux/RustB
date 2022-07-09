#!/bin/bash

docker run -it --rm --volume $(pwd)/code:/src --volume $(pwd)/bashrc:/root/.bashrc rust bash
