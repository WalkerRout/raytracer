#!/bin/bash

PROJ_NAME=raytracer

cd build/
#cmake .. -DCMAKE_CXX_FLAGS="-O3"

if [ "$1" == "run" ]
then
  make "$PROJ_NAME"_run; src/"$PROJ_NAME"_run
else
  make "$PROJ_NAME"_test; test/"$PROJ_NAME"_test
fi

cd ..
