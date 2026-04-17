#!/bin/bash
set -e
sed -i 's/^#ParallelDownloads = 5/ParallelDownloads = 10/' /etc/pacman.conf

# --- 1. IDENTITY & NETWORK ---
echo "haremalos" > /etc/hostname
cat <<HOSTS > /etc/hosts
127.0.0.1 localhost
::1 localhost
127.0.1.1 haremalos.localdomain haremalos
HOSTS

# --- 2. KEYRING & REPOS ---
pacman -Sy --noconfirm archlinux-keyring
pacman-key --init
pacman-key --populate archlinux
pacman -Syu --noconfirm

# --- 3. GRUB & BOOTLOADER ---
pacman -S --noconfirm --needed grub efibootmgr os-prober ntfs-3g
sed -i 's/^#\(GRUB_DISABLE_OS_PROBER=false\)/\1/' /etc/default/grub
sed -i 's/GRUB_CMDLINE_LINUX_DEFAULT="\(.*\)"/GRUB_CMDLINE_LINUX_DEFAULT="\1 amdgpu.dc=1"/' /etc/default/grub

grub-install --target=x86_64-efi --efi-directory=/boot --bootloader-id=GRUB --recheck
export GRUB_DISABLE_OS_PROBER=false
grub-mkconfig -o /boot/grub/grub.cfg

# --- 4. SECURITY (Sudo & Polkit) ---
echo '%wheel ALL=(ALL) NOPASSWD: ALL' > /etc/sudoers.d/wheel-nopasswd
chmod 440 /etc/sudoers.d/wheel-nopasswd

mkdir -p /etc/polkit-1/rules.d
cat <<POLKIT > /etc/polkit-1/rules.d/49-nopasswd.rules
polkit.addRule(function(action, subject) {
    if (subject.isInGroup("wheel")) {
        return polkit.Result.YES;
    }
});
POLKIT

# --- 5. THE SKELETON & ENVIRONMENT ---
mkdir -p /etc/skel/Media/{Documents,Pictures,Videos,Music,Downloads} 
mkdir -p /etc/skel/Settings/{Config,Data} /etc/skel/Projects
mkdir -p /etc/skel/Settings/Config/
cat <<HOME >> /etc/security/pam_env.conf
XDG_CONFIG_HOME DEFAULT=@{HOME}/Settings/Config
XDG_DATA_HOME DEFAULT=@{HOME}/Settings/Data
HOME
cat <<DIRS > /etc/skel/.bash_profile
export XDG_DOCUMENTS_DIR="\$HOME/Media/Documents"
export XDG_PICTURES_DIR="\$HOME/Media/Pictures"
export XDG_VIDEOS_DIR="\$HOME/Media/Videos"
export XDG_MUSIC_DIR="\$HOME/Media/Music"
export XDG_DOWNLOAD_DIR="\$HOME/Media/Downloads"
export XCURSOR_THEME="Bibata-Modern-Classic"
export XCURSOR_SIZE=24
DIRS
cat <<'USERDIRS' > /etc/skel/Settings/Config/user-dirs.dirs
XDG_DESKTOP_DIR="$HOME"
XDG_DOWNLOAD_DIR="$HOME/Media/Downloads"
XDG_TEMPLATES_DIR="$HOME"
XDG_PUBLICSHARE_DIR="$HOME"
XDG_DOCUMENTS_DIR="$HOME/Media/Documents"
XDG_MUSIC_DIR="$HOME/Media/Music"
XDG_PICTURES_DIR="$HOME/Media/Pictures"
XDG_VIDEOS_DIR="$HOME/Media/Videos"
XDG_PROJECTS_DIR="$HOME/Projects"
USERDIRS

# --- 6. XDG & USER ACCESS ---
mkdir -p /etc/skel/Settings/Config/xdg-desktop-portal
cat <<PORTALS > /etc/skel/Settings/Config/xdg-desktop-portal/portals.conf
[preferred]
default=hyprland
org.freedesktop.impl.portal.FileChooser=gtk
PORTALS

mkdir -p /etc/default
cat <<ACC > /etc/default/useradd
GROUP=1000
GROUPS=wheel,video,render,storage,power,input
HOME=/home
SHELL=/bin/bash
SKEL=/etc/skel
CREATE_MAIL_SPOOL=no
ACC

# --- 6. DRIVER RULES & INIT ---
touch /etc/vconsole.conf
sed -i 's/^HOOKS=.*/HOOKS=(base udev autodetect modconf kms block filesystems keyboard fsck)/' /etc/mkinitcpio.conf
sed -i 's/^MODULES=.*/MODULES=(amdgpu)/' /etc/mkinitcpio.conf
mkinitcpio -P

# --- 7. CHAOTIC AUR & OPT FOLDER ---
sed -i '/\[multilib\]/,/Include/s/^#//' /etc/pacman.conf
pacman-key --recv-key 3056513887B78AEB --keyserver keyserver.ubuntu.com
pacman-key --lsign-key 3056513887B78AEB
pacman -U --noconfirm 'https://cdn-mirror.chaotic.cx/chaotic-aur/chaotic-keyring.pkg.tar.zst'
pacman -U --noconfirm 'https://cdn-mirror.chaotic.cx/chaotic-aur/chaotic-mirrorlist.pkg.tar.zst'
pacman-key --populate chaotic
printf "\n[chaotic-aur]\nInclude = /etc/pacman.d/chaotic-mirrorlist\n" >> /etc/pacman.conf
pacman -Syu --noconfirm
mkdir -p /opt

# --- 8. SWAP FILE ---
if [ ! -f /swapfile ]; then
    fallocate -l 8G /swapfile
    chmod 600 /swapfile
    mkswap /swapfile
    swapon /swapfile || true
    echo "/swapfile none swap defaults 0 0" >> /etc/fstab
fi
