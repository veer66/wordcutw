#!/bin/sh
mkdir -p headers
cbindgen --lang c  --crate wordcutw --output headers/wordcutw.h
