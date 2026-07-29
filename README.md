lang en_US.UTF-8
keyboard us
timezone UTC
bootloader --location=mbr
clearpart --all --initlabel
autopart

%packages
@gnome-desktop
steam
blender
obs-studio
ardour7
bitwarden
%end

%post
systemctl enable bluetooth
%end
