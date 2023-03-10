#!/usr/bin/env bash

readonly BASE_DIR=$(dirname $(readlink -f "$0"))
readonly BUILD_DIR="${BASE_DIR}/exports"

readonly GODOT="/opt/godot/Godot_v4.0-stable_linux.x86_64"
readonly TARGET="Web"

mkdir -p ${BUILD_DIR}

${GODOT} --headless --export-release "${TARGET}" ${BASE_DIR}/project.godot

# NOTE: Workaround for first build always failing with half baked artifacts
${GODOT} --headless --export-release "${TARGET}" ${BASE_DIR}/project.godot
