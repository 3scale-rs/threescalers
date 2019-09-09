#!/bin/bash
#
# This script makes sure the dependencies in Cargo.toml adhere to the
# requirements in the dependency licenses' whitelist.
#
# You can run it locally if you have license_finder installed, and if
# you don't have it will try to use a container.
#
# If you need a different docker binary or elevated privileges just
# invoke this script like this:
#
# $ DOCKER="sudo /path/to/docker" license-check.sh
#
# You can also specify DECISIONS_FILE for a different license whitelist.
#
# Check the variables below for overrides you can use.

SCRIPT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
ROOT_DIR=$(dirname "${SCRIPT_DIR}")

# Overrideable variables

# license_finder executable name
LF="${LF:-license_finder}"
# License whitelist file relative to ROOT_DIR and always contained in it.
DECISIONS_FILE="${DECISIONS_FILE:-"ci/dependency-decisions.yml"}"
# Extra arguments for license_finder (other than the decisions file)
LF_ARGS=

run() {
    local license_finder="${1}"

    ${license_finder} \
        --decisions-file="${ROOT_DIR}/${DECISIONS_FILE}" ${LF_ARGS}
}

run_on_docker()
{
    local docker=${DOCKER:-$(which docker 2> /dev/null)}

    if [[ -n "${docker}" ]]; then
      ${docker} run -v "${ROOT_DIR}":/scan:z -it \
          -e DECISIONS_FILE="${DECISIONS_FILE}" \
          -e LF_ARGS="${LF_ARGS}" \
          licensefinder/license_finder \
          /bin/bash -c "cd /scan && ci/license-check.sh"
    else
      echo "Please install license_finder or docker to use this tool."
    fi
}

main() {
  local license_finder="$(which "${LF}" 2> /dev/null)"

  if [[ -n "${license_finder}" ]]; then
    run "${license_finder}"
  else
    echo "* ${LF} not found locally, using a container..."
    run_on_docker
  fi
}

main "${@}"
