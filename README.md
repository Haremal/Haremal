#!/bin/bash
set -euo pipefail
cd "$(dirname "$0")"
chmod +x ./*.sh

# --- 1. PRE-FLIGHT ---
if [ ! -d /sys/firmware/efi ]; then echo "Error: UEFI required."; exit 1; fi
swapoff -a 2>/dev/null || true
umount -R /mnt 2>/dev/null || true

# --- 2. DISK SELECTION ---
echo "-------------------------------------------------------"
lsblk -d -n -o NAME,SIZE,MODEL
echo "-------------------------------------------------------"
read -p "TARGET DISK (e.g. /dev/sda or /dev/nvme0n1): " TARGET_DISK

echo "-------------------------------------------------------"
lsblk "$TARGET_DISK" -o NAME,SIZE,TYPE,PARTLABEL,PARTTYPENAME
echo "-------------------------------------------------------"
echo "IDENTIFY YOUR PARTITIONS"
read -p "Type the EFI partition (e.g., /dev/sda1): " EFI_P
read -p "Type the ROOT partition (e.g., /dev/sda2): " ROOT_P
read -p "Type the HOME partition (e.g., /dev/sda3): " HOME_P

read -p "Enter Root partition size (e.g., 50G) or press Enter to skip creation: " ROOT_SIZE
read -p "TYPE 'YES' TO CONTINUE AND APPLY CHANGES TO $TARGET_DISK: " FINAL_CHECK
[[ "$FINAL_CHECK" != "YES" ]] && exit 1

# --- 3. CREATE FILESYSTEMS ---
E_NUM=$(echo "$EFI_P" | grep -oE '[0-9]+$')
R_NUM=$(echo "$ROOT_P" | grep -oE '[0-9]+$')
H_NUM=$(echo "$HOME_P" | grep -oE '[0-9]+$')
sgdisk -n "$E_NUM":0:+512M -t "$E_NUM":ef00 -c "$E_NUM":EFI "$TARGET_DISK" || true
[[ -n "$ROOT_SIZE" ]] && sgdisk -n "$R_NUM":0:+"$ROOT_SIZE" -t "$R_NUM":8300 -c "$R_NUM":Arch_Root "$TARGET_DISK" || true
sgdisk -n "$H_NUM":0:0 -t "$H_NUM":8300 -c "$H_NUM":Arch_Home "$TARGET_DISK" || true

udevadm settle
partprobe "$TARGET_DISK"
sleep 2

# --- 4. APPLY FILESYSTEMS ---
[[ "$(lsblk -no FSTYPE "$EFI_P" || echo "none")" != "vfat" ]] && mkfs.fat -F 32 "$EFI_P"
[[ "$(lsblk -no FSTYPE "$HOME_P" || echo "none")" != "ext4" ]] && mkfs.ext4 -F "$HOME_P"
wipefs -af "$ROOT_P" && mkfs.ext4 -F "$ROOT_P"

# --- 5. MOUNT FILESYSTEMS ---
mount "$ROOT_P" /mnt
mkdir -p /mnt/boot /mnt/home
mount "$EFI_P" /mnt/boot
mount "$HOME_P" /mnt/home

# --- 6. CHOICES & INSTALLS ---
read -p "Install Steam? (y/n) " I_STEAM
read -p "Install Fyrox? (y/n) " I_FYROX
read -p "Install Blender? (y/n) " I_BLENDER
read -p "Install OBS? (y/n) " I_OBS
read -p "Install Ardour? (y/n) " I_ARDOUR
read -p "Install Bitwarden? (y/n) " I_BITWARDEN
pacstrap -K /mnt base linux linux-firmware sudo curl wget amd-ucode bash-completion --noconfirm --needed

# --- 7. CHROOT HANDOFF ---
genfstab -U /mnt >> /mnt/etc/fstab
arch-chroot /mnt /bin/bash <<EOF
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
mkdir -p /etc/skel/Projects
cat <<DIRS > /etc/skel/.bash_profile
export XCURSOR_THEME="Bibata-Modern-Classic"
export XCURSOR_SIZE=24
DIRS
cat <<'USERDIRS' > /etc/skel/Settings/Config/user-dirs.dirs
XDG_DESKTOP_DIR="$HOME"
XDG_TEMPLATES_DIR="$HOME"
XDG_PUBLICSHARE_DIR="$HOME"
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


# --- 7. SWAP FILE ---
if [ ! -f /swapfile ]; then
    fallocate -l 8G /swapfile
    chmod 600 /swapfile
    mkswap /swapfile
    swapon /swapfile || true
    echo "/swapfile none swap defaults 0 0" >> /etc/fstab
fi

# --- HAREMAL APPS ---
pacman -Syu
mkdir -p /opt

pacman -S --noconfirm --needed pinta
[[ "${I_STEAM:-N}" =~ [Yy] ]] && pacman -S --noconfirm --needed steam gamescope
[[ "${I_BLENDER:-N}" =~ [Yy] ]] && pacman -S --noconfirm --needed blender
[[ "${I_FYROX:-N}" =~ [Yy] ]] && cargo install fyrox-project-manager
[[ "${I_OBS:-N}" =~ [Yy] ]] && pacman -S --noconfirm --needed obs-studio
[[ "${I_ARDOUR:-N}" =~ [Yy] ]] && pacman -S --noconfirm --needed ardour
[[ "${I_BITWARDEN:-N}" =~ [Yy] ]] && pacman -S --noconfirm --needed bitwarden bitwarden-cli

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
EOF

echo 'export CARGO_HOME="/opt/cargo"' > /mnt/etc/profile.d/cargo.sh
echo 'export PATH="/opt/cargo/bin:$PATH"' >> /mnt/etc/profile.d/cargo.sh
chmod +x /mnt/etc/profile.d/cargo.sh

rm /mnt/*.sh
echo "SUCCESS: Fresh install complete."
