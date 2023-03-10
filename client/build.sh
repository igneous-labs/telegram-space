#!/usr/bin/env bash

readonly BASE_DIR=$(dirname $(readlink -f "$0"))
readonly BUILD_DIR="${BASE_DIR}/exports"

# NB: change this to where godot_v4.0-stable binary is installed
# NOTE: make sure the export template for the same version is available (see README)
readonly GODOT="/path/to/godot/binary"
readonly TARGET="Web"

mkdir -p ${BUILD_DIR}

${GODOT} --headless --export-debug "${TARGET}" ${BASE_DIR}/project.godot
