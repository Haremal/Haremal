# Basic Fedora GNOME automated setup
lang en_US.UTF-8
keyboard us
timezone UTC
bootloader --location=mbr
clearpart --all --initlabel
autopart

# Pick your packages directly
%packages
@gnome-desktop
steam
blender
obs-studio
ardour7
bitwarden
%end

# Run custom bash post-install setup
%post
# Custom configs, cargo paths, or systemd tweaks go here
systemctl enable bluetooth
%end
