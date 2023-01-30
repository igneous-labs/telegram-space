#!/usr/bin/env bash

readonly BASE_DIR=$(dirname $(readlink -f "$0"))
readonly BUILD_DIR="${BASE_DIR}/exports"

# NB: change this to where godot_v4.0-beta16 binary is installed
readonly GODOT="/opt/godot/Godot_v4.0-beta16_linux.x86_64"
readonly TARGET="Web"

mkdir -p ${BUILD_DIR}

# NOTE: Workaround for: https://github.com/godotengine/godot/issues/72360
mkdir -p ${BASE_DIR}/.godot/{imported,editor}

${GODOT} --headless --export-debug "${TARGET}" ${BASE_DIR}/project.godot

# NOTE: Workaround for first build always failing  with half baked artifacts without .godot/ directory
${GODOT} --headless --export-debug "${TARGET}" ${BASE_DIR}/project.godot
