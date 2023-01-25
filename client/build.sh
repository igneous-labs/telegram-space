#!/usr/bin/env bash

readonly BASE_DIR=$(dirname $(readlink -f "$0"))
readonly BUILD_DIR="${BASE_DIR}/exports"

# NB: change this to where godot_v4.0-beta14 binary is installed
readonly GODOT="/opt/godot/Godot_v4.0-beta14_linux.x86_64";
readonly TARGET="Web"

mkdir -p ${BUILD_DIR}

${GODOT} --headless --export-debug "${TARGET}" ${BASE_DIR}/project.godot
