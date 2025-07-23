#!/usr/bin/env bash

# @describe Install a binary release of {{ name }} hosted on GitHub
# @meta inherit-flag-options
# @meta require-tools curl,install,mktemp,tar
# @flag   -D --debug                            Enable debug mode
# @flag   -f --force                            Force overwriting an existing binary
# @option    --tag=latest                       Tag (version) of the binary to install
# @option    --bin-dir=/usr/local/bin <DIR>     Where to install the binary

set -eo pipefail

## Color functions
##
## Usage:
## Use any of the functions below to color or format a portion of a string.
##
##   echo "before $(red this is red) after"
##   echo "before $(green_bold this is green_bold) after"
##
## Color output will be disabled if `NO_COLOR` environment variable is set
## in compliance with https://no-color.org/
##
print_in_color() {
    local color="$1"
    shift
    if [[ -z ${NO_COLOR+x} ]]; then
        printf "${color}%b\e[0m\n" "$*"
    else
        printf "%b\n" "$*"
    fi
}
red() { print_in_color "\e[31m" "$*"; }
green() { print_in_color "\e[32m" "$*"; }
yellow() { print_in_color "\e[33m" "$*"; }
blue() { print_in_color "\e[34m" "$*"; }
magenta() { print_in_color "\e[35m" "$*"; }
cyan() { print_in_color "\e[36m" "$*"; }
bold() { print_in_color "\e[1m" "$*"; }
underlined() { print_in_color "\e[4m" "$*"; }
red_bold() { print_in_color "\e[1;31m" "$*"; }
green_bold() { print_in_color "\e[1;32m" "$*"; }
yellow_bold() { print_in_color "\e[1;33m" "$*"; }
blue_bold() { print_in_color "\e[1;34m" "$*"; }
magenta_bold() { print_in_color "\e[1;35m" "$*"; }
cyan_bold() { print_in_color "\e[1;36m" "$*"; }
red_underlined() { print_in_color "\e[4;31m" "$*"; }
green_underlined() { print_in_color "\e[4;32m" "$*"; }
yellow_underlined() { print_in_color "\e[4;33m" "$*"; }
blue_underlined() { print_in_color "\e[4;34m" "$*"; }
magenta_underlined() { print_in_color "\e[4;35m" "$*"; }
cyan_underlined() { print_in_color "\e[4;36m" "$*"; }

# Project metadata
readonly NAME="{{ name }}"
readonly URL="{{ url }}"
readonly REPO="${URL#https://github.com/}"

# Temporary directory for the installation process
temp_dir=""

chomp() { printf "%s" "${1/"$'\n'"/}"; }

oh1() {
    printf "$(green "==>") $(bold "%s")\n" "$*"
}

oh2() {
    printf "$(blue "==>") $(bold "%s")\n" "$*"
}

warn() {
    printf "$(yellow_bold Warning): %s\n" "$(chomp "$1")" >&2
}

error() {
    printf "$(red_bold Error): %s\n" "$(chomp "$1")" >&2
}

fatal() {
    if [[ -n "${temp_dir}" ]]; then
        rm -rf "${temp_dir}"
    fi
    error "$@"
    exit 1
}

need() {
    if ! command -v "$1" >/dev/null 2>&1; then
        err "need $1 (command not found)"
    fi
}

main() {
    oh1 "Installing $(green "${NAME}") from ${URL}"

    local force="${argc_force:-0}"
    local bin_dir="${argc_bin_dir:-/usr/local/bin}"
    local tag="${argc_tag:-latest}"
    if [[ "${tag}" == "latest" ]]; then
        need jq
        tag="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | jq -r .tag_name)"
    fi
    local archive="${URL}/releases/download/${tag}/${NAME}-${tag}.tar.gz"

    oh2 "Showing information for $(green "${NAME}") installation"
    echo "Repository:  ${URL}"
    echo "Tag:         ${tag}"
    echo "Name:        ${NAME}"
    echo "Archive:     ${archive}"
    echo "Destination: ${bin_dir}"

    temp_dir="$(mktemp -d || mktemp -d -t tmp)"
    local temp_file="${temp_dir}/${NAME}.tar.gz"
    curl -fsSL -o "${temp_file}" "${archive}"

    oh2 "Extracting ${temp_file} archive to ${temp_dir}"
    tar -C ${temp_dir} -zxvf "${temp_file}"

    local temp_bin_dir="${temp_dir}/bin"
    if [[ ! -d "${temp_bin_dir}" ]]; then
        fatal "No 'bin' directory found in the archive."
    fi

    oh2 "Installing binaries from ${temp_bin_dir} to ${bin_dir}"
    for p in "${temp_bin_dir}"/*; do
        local filename="$(basename "$p")"
        if [[ -e "${bin_dir}/${filename}" ]] && [[ "${force}" == "0" ]]; then
            fatal "${filename} already exists in ${bin_dir}. Use --force dest overwrite."
        else
            install -v -m 755 "$p" "${bin_dir}"
        fi
    done

    oh2 "Cleaning up temporary files"
    rm -rf "${temp_dir}"
}

_argc_before() {
    if [[ "${argc_debug:-}" == "1" ]]; then
        set -x
    fi
}

eval "$(argc --argc-eval "$0" "$@")"
