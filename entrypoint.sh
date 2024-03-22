#!/bin/bash
set -e

exec gosu test_user /app/vtechlabs "$@"
