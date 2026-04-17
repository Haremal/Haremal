#!/bin/bash
set -exuo pipefail
pacman -Syu --noconfirm

# --- 1. BASE SYSTEM & DRIVERS ---
# Get the hardware ready before the heavy apps
pacman -S --noconfirm --needed \
	mesa lib32-mesa vulkan-radeon lib32-vulkan-radeon \
	libva-mesa-driver lib32-libva-mesa-driver libva-utils \
	pipewire pipewire-pulse pipewire-alsa pipewire-jack \
	networkmanager bluez bluez-utils glib2 fontconfig \
	xdg-desktop-portal xdg-desktop-portal-gnome xdg-desktop-portal-gtk \
	wayland wayland-protocols xorg-server-xwayland xwayland-satellite \
	polkit hyprpolkitagent xdg-utils libsecret gnome-keyring \
	gsettings-desktop-schemas qt5-wayland qt6-wayland qt5ct qt6ct

# --- 2. RUST ---
pacman -S --noconfirm --needed \
	base-devel git socat ninja jq \
	rust rust-analyzer clang mold xdotool
export CARGO_HOME="/opt/cargo"
export PATH="$CARGO_HOME/bin:$PATH"
cargo install dioxus-cli cargo-watch cargo-bundle
chgrp -R users /opt/cargo
chmod -R g+w /opt/cargo
chmod -R 2775 /opt/cargo
chmod -R -s /opt/cargo/bin/
chmod -R a+rx /opt/cargo/bin/

# --- 3. THE HAREMAL OS STACK (Core Apps) ---
pacman -S --noconfirm --needed \
	ly niri awww wallust-git hypridle hyprlock \
	rio helix yazi bottom cava neo-matrix-git \
	mpv ffmpeg ouch wl-clipboard-rs libnotify wireplumber \
	brightnessctl lm_sensors fd ripgrep papirus-icon-theme bibata-cursor-theme

# --- 4. OTHERS ---
pacman -Rdd --noconfirm nautilus
pacman -S --noconfirm --needed pinta

# --- 5. CHOSEN APPS ---
[[ "${I_STEAM:-N}" =~ [Yy] ]] && pacman -S --noconfirm --needed steam gamescope
[[ "${I_BLENDER:-N}" =~ [Yy] ]] && pacman -S --noconfirm --needed blender
[[ "${I_FYROX:-N}" =~ [Yy] ]] && cargo install fyrox-project-manager
[[ "${I_OBS:-N}" =~ [Yy] ]] && pacman -S --noconfirm --needed obs-studio
[[ "${I_ARDOUR:-N}" =~ [Yy] ]] && pacman -S --noconfirm --needed ardour
[[ "${I_BITWARDEN:-N}" =~ [Yy] ]] && pacman -S --noconfirm --needed bitwarden bitwarden-cli
