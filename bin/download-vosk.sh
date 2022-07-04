#!/bin/bash

set -ue

mkdir -p lib/vosk
mkdir -p .cache/
cd .cache

if [ `uname -s` == 'Linux' ]; then
	filename=vosk-linux-x86_64-0.3.42
	wget --no-clobber https://github.com/alphacep/vosk-api/releases/download/v0.3.42/$filename.zip -O $filename.zip || true
	unzip -n $filename.zip
	cp -vr $filename/* ../lib/vosk
fi
