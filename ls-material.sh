#!/bin/bash

# Define icons for different file types using Nerd Fonts
declare -A ICONS

# Directory icon
ICONS["dir"]=""

# Common file types
ICONS["txt"]=""
ICONS["pdf"]=""
ICONS["zip"]=""
ICONS["tar"]=""
ICONS["gz"]=""
ICONS["mp3"]=""
ICONS["mp4"]=""
ICONS["jpg"]=""
ICONS["png"]=""
ICONS["gif"]=""
ICONS["py"]=""
ICONS["js"]=""
ICONS["html"]=""
ICONS["css"]=""
ICONS["sh"]=""
ICONS["c"]=""
ICONS["cpp"]=""
ICONS["java"]=""
ICONS["go"]=""
ICONS["rb"]=""
ICONS["rs"]=""
ICONS["php"]=""
ICONS["json"]=""
ICONS["md"]=""

# Default icon
ICONS["default"]=""

# Hidden file icon
ICONS["hidden"]=""

# Function to get icon based on file type
get_icon() {
    local filename="$1"
    
    # Check if it's a directory
    if [[ -d "$filename" ]]; then
        echo -e "\e[34m${ICONS["dir"]}\e[0m"
        return
    fi
    
    # Check if it's a hidden file (starts with .)
    if [[ "$filename" == .* ]]; then
        echo -e "\e[36m${ICONS["hidden"]}\e[0m"
        return
    fi

    # Extract file extension
    local ext="${filename##*.}"

    # Check if the extension has an icon
    if [[ -n "${ICONS[$ext]}" ]]; then
        echo -e "\e[32m${ICONS[$ext]}\e[0m"
    else
        echo -e "\e[90m${ICONS["default"]}\e[0m"
    fi
}

# List files and prepend icons
for file in $(ls -A --color=never); do
    icon=$(get_icon "$file")
    echo -e "$icon $file"
done
