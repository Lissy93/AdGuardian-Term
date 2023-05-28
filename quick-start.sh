#!/usr/bin/env bash

#################################################################
# 🛡️ AdGuardian-Term - Simple start-up script                   #
#################################################################
# This scipt will download the latest binary for your system,   #
# from GitHub, check it, make it executable, and then run it.   #
#                                                               #
# Docs and source: https://github.com/Lissy93/AdGuardian-Term   #
# Licensed: MIT (C) Alicia Sykes 2022 <https://aliciasykes.com> #
#################################################################

# Config
upstream_repo="https://github.com/Lissy93/AdGuardian-Term"
adguardian_version="latest"
download_location="./adguardian"

# Colours and formatting
NO_FORMAT="\033[0m"
F_BOLD="\033[1m"
C_RED="\033[38;5;9m"
C_AQUA="\033[38;5;12m"
C_YELLOW="\033[38;5;11m"

# Functions for printing stuff
function exit_script {
  echo -e "${F_BOLD}${C_RED}Error: ${1}${NO_FORMAT}"
  echo -e "${C_YELLOW}Unable to continue, but not to worry, there are alternate installation methods for your system."
  echo -e "See the docs, at: ${upstream_repo} for help.${NO_FORMAT}"
  exit 1
}

function print_heading {
  echo -e "\n${F_BOLD}${C_AQUA}${1}${NO_FORMAT}"
}
function print_info {
  echo -e "${C_AQUA}\033[2m${1}${NO_FORMAT}"
}

# Check what system the user is running
print_heading "Checking system type"
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
  print_info "System type: Linux"
  bin_target="adguardian-linux"
elif [[ "$OSTYPE" == "darwin"* ]]; then
  print_info "System type: Apple OS X"
  bin_target="adguardian-macos"
elif [[ "$OSTYPE" == "cygwin" ]]; then
  print_info "System type: Windows/Cygwin"
  bin_target="adguardian-windows.exe"
else
    exit_script "Unsupported System"
fi

# Make the download link to latest binary for users system type
download_link="$upstream_repo/releases/$adguardian_version/download/$bin_target"

# Check if the binary already exists
print_heading "Preparing to download"
if [ -f "$download_location" ]; then
    print_info "File already exists, skipping download."
else
    # Download with either curl or wget, depending on what is installed
    if hash "curl" 2> /dev/null; then
      print_info "Downloading to $download_location (with curl)"
      curl -L -o $download_location $download_link
    elif hash "wget" 2> /dev/null; then
      print_info "Downloading to $download_location (with wget)"
      wget \
        --no-verbose --show-progress \
        --progress=dot:mega -q -S \
        -O $download_location $download_link
    else
      exit_script "Neither curl nor wget were found on your system"
    fi
fi

# Make the binary executable, then run the application
print_heading "Preparing to run"
print_info "Updating permissions for $download_location"
chmod +x $download_location
print_info "Starting AdGuardian....\n\n"
$download_location
