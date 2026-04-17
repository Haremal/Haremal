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
cp system/setup.sh system/apps.sh system/config.sh system/finish.sh /mnt/
chmod +x /mnt/*.sh
arch-chroot /mnt /bin/bash <<EOF
  export I_STEAM="${I_STEAM}"
  export I_FYROX="${I_FYROX}"
  export I_BLENDER="${I_BLENDER}"
  export I_OBS="${I_OBS}"
  export I_ARDOUR="${I_ARDOUR}"
  export I_BITWARDEN="${I_BITWARDEN}"
  /setup.sh
  /apps.sh
  /config.sh
  /finish.sh
EOF

echo 'export CARGO_HOME="/opt/cargo"' > /mnt/etc/profile.d/cargo.sh
echo 'export PATH="/opt/cargo/bin:$PATH"' >> /mnt/etc/profile.d/cargo.sh
chmod +x /mnt/etc/profile.d/cargo.sh

rm /mnt/*.sh
echo "SUCCESS: Fresh install complete."
