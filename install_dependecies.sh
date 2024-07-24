#!/bin/bash

# Function to check the Linux distribution
detect_distro() {
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        DISTRO=$ID
    elif [ -f /etc/redhat-release ]; then
        DISTRO="rhel"
    elif [ -f /etc/debian_version ]; then
        DISTRO="debian"
    else
        DISTRO=$(uname -s)
    fi
}

# Function for installing packages on Debian/Ubuntu-based distributions
install_debian() {
    sudo apt-get update
    sudo apt-get install -y pkg-config libssl-dev libmariadb-dev libmysqld-dev libsmbclient-dev libsmbclient libpq-dev
}

# Function for installing packages on Fedora/RHEL-based distributions
install_rhel() {
    sudo dnf install -y pkg-config openssl-devel mariadb-devel mysql-devel libsmbclient-devel libsmbclient postgresql-devel
}

# Function for installing packages on Arch Linux-based distributions
install_arch() {
    sudo pacman -Sy --needed pkg-config openssl mariadb mysql smbclient postgresql-libs
}

# Detects the Linux distribution
detect_distro

# Executes the appropriate installation command based on the distribution detected
case $DISTRO in
    ubuntu|debian|kali)
        echo "Detected Debian/Ubuntu based system."
        install_debian
        ;;
    fedora|rhel|centos)
        echo "Detected Fedora/RHEL/CentOS based system."
        install_rhel
        ;;
    arch)
        echo "Detected Arch Linux based system."
        install_arch
        ;;
    *)
        echo "Unsupported distribution: $DISTRO"
        exit 1
        ;;
esac

echo "Installation completed."

cargo install --git https://github.com/Jsmoreira02/Hazard.git
