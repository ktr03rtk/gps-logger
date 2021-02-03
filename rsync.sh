#!/bin/bash

cd $(dirname $0)
dir=$(basename `pwd`)

fswatch -e "target/" -o . | xargs -I{} rsync -av --delete --exclude="target/" . pi@raspberrypi.local:work/${dir}
