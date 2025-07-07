#!/usr/bin/env bash

set -eux

CSDIR="$(dirname "$(realpath "$0")")"

cd "$CSDIR"

uv venv -p 3.12
source .venv/bin/activate

uv pip install psycopg2-binary
