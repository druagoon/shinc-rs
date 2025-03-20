#!/usr/bin/env bash

if [[ -z "${NEW_VERSION}" ]]; then
    echo "NEW_VERSION environment variable is not set!"
    exit 1
fi

git cliff -o CHANGELOG.md --tag "${NEW_VERSION}"
