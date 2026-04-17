#!/bin/bash
set -e

# --- SYSTEM SET --- 
ln -sf /usr/share/zoneinfo/UTC /etc/localtime
hwclock --systohc
echo "en_US.UTF-8 UTF-8" > /etc/locale.gen && locale-gen
echo "LANG=en_US.UTF-8" > /etc/locale.conf
mkdir -p /usr/share/glib-2.0/schemas/
cat <<CRS > /usr/share/glib-2.0/schemas/99_haremalos_defaults.gschema.override
[org.gnome.desktop.interface]
color-scheme='prefer-dark'
icon-theme='Papirus-Dark'
cursor-theme='Bibata-Modern-Classic'
cursor-size=24
CRS
glib-compile-schemas /usr/share/glib-2.0/schemas/
timedatectl set-ntp true

# --- NIRI SESSION ---
mkdir -p /usr/share/wayland-sessions
cat <<ENTRY > /usr/share/wayland-sessions/niri.desktop
[Desktop Entry]
Name=Niri
Comment=A scrollable-tiling Wayland compositor
Exec=niri-session
Type=Application
ENTRY

# --- ENVIRONMENT VARIABLES ---
mkdir -p /etc/skel/Settings/Config/environment.d
cat <<ENV > /etc/skel/Settings/Config/environment.d/haremalos.conf
# Toolkits
QT_QPA_PLATFORM=wayland
SDL_VIDEODRIVER=wayland
CLUTTER_BACKEND=wayland
# Theming
QT_QPA_PLATFORMTHEME=qt5ct
# Graphics (AMD Optimized)
LIBVA_DRIVER_NAME=radeonsi
VDPAU_DRIVER=radeonsi
ENV

# --- SERVICES ---
systemctl disable getty@tty2.service
systemctl enable ly@tty2.service
systemctl enable NetworkManager
systemctl enable bluetooth.service
systemctl enable fstrim.timer
sed -i 's/#AutoEnable=false/AutoEnable=true/' /etc/bluetooth/main.conf 2>/dev/null || true

# --- SENSORS ---
if [ -d /sys/class/dmi ]; then
yes | sensors-detect --auto > /dev/null 2>&1 || true
fi

# --- FIRST BOOT ---
passwd -d root
chage -d 0 root
cat <<ISSUE > /etc/issue
------------------------------------------------------
WELCOME TO HAREMALOS (FIRST BOOT)
------------------------------------------------------
Login as 'root' (No password required).
Then set password for root.

Then run:
1. useradd -m -c "Display Name" yourname
2. passwd yourname
3. rm /etc/issue && reboot

(This message will disappear after reboot)
------------------------------------------------------
ISSUE
