# Configuration file for lsf (ls with icons)
# This file can be sourced to set up lsf aliases for your shell

# Function to set up lsf alias based on current shell
setup_lsf_alias() {
    # Check if lsf command exists
    if command -v lsf >/dev/null 2>&1; then
        # Determine the current shell properly
        if [ -n "$BASH_VERSION" ]; then
            # Bash shell
            alias ls='lsf'
        elif [ -n "$ZSH_VERSION" ]; then
            # Zsh shell
            alias ls='lsf'
        else
            # For other shells, set up the basic alias
            alias ls='lsf'
        fi
        echo "lsf alias has been set up for your shell!"
        echo "Now 'ls' command will use 'lsf' with icons."
        echo "Note: For fish shell, manually add 'alias ls=\"lsf\"' to ~/.config/fish/config.fish"
    else
        echo "Error: lsf command not found. Please install lsf first."
    fi
}

# Auto-setup when this file is sourced
setup_lsf_alias