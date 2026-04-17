#!/bin/bash
set -e

# FONTS
mkdir -p /usr/share/fonts/TTF /etc/skel/Settings/fontconfig
curl -L -o /usr/share/fonts/TTF/Monocraft.ttc https://github.com/IdreesInc/Monocraft/releases/latest/download/Monocraft.ttc
cat <<FONT > /etc/skel/Settings/fontconfig/fonts.conf
<?xml version="1.0"?>
<!DOCTYPE fontconfig SYSTEM "urn:fontconfig:fonts.dtd">
<fontconfig>
	<match target="pattern">
		<test qual="any" name="family"><string>system</string></test>
<edit name="family" mode="assign" binding="strong"><string>Monocraft</string></edit>
	</match>
</fontconfig>
FONT
fc-cache -fv

# EWW
mkdir -p /etc/skel/Settings/Config/eww
cat <<EWW > /etc/skel/Settings/Config/eww/eww.scss
* {
    font-family: "system";
}
EWW

# NIRI
mkdir -p /etc/skel/Settings/Config/niri
cat <<NIRI > /etc/skel/Settings/Config/niri/config.kdl
input {
    keyboard {
        xkb {
            layout "us"
        }
    }
    touchpad {
        tap
        dwt
        natural-scroll
    }
}

output {
    mode-action {
        scale 1.0
    }
}

layout {
    gaps 16
    center-focused-column "never"

    preset-column-widths {
        proportion 0.33333
        proportion 0.5
        proportion 0.66667
    }

    default-column-width { proportion 0.5; }

    focus-ring {
        width 2
        active-color "#7aa2f7"
        inactive-color "#565f89"
    }
}

// startups {
spawn-at-startup "awww-daemon"
spawn-at-startup "hypridle"
spawn-at-startup "/usr/lib/hyprpolkitagent"
spawn-at-startup "dbus-update-activation-environment" "--systemd" "WAYLAND_DISPLAY" "XDG_CURRENT_DESKTOP=niri"
// }

binds {
    Mod+Shift+E { quit; }
    Mod+Q { close-window; }
    Mod+Space { spawn "rio"; }
    Print { spawn "sh" "-c" "grim -g \"\$(slurp)\" - | wl-copy"; }
    XF86AudioRaiseVolume allow-when-locked=true { spawn "wpctl" "set-volume" "@DEFAULT_AUDIO_SINK@" "5%+"; }
    XF86AudioLowerVolume allow-when-locked=true { spawn "wpctl" "set-volume" "@DEFAULT_AUDIO_SINK@" "5%-"; }
    XF86AudioMute        allow-when-locked=true { spawn "wpctl" "set-mute" "@DEFAULT_AUDIO_SINK@" "toggle"; }
    XF86MonBrightnessUp   allow-when-locked=true { spawn "brightnessctl" "set" "5%+"; }
    XF86MonBrightnessDown allow-when-locked=true { spawn "brightnessctl" "set" "5%-"; }
    Mod+Left  { focus-column-left; }
    Mod+Right { focus-column-right; }
    Mod+WheelScrollDown { focus-column-right; }
    Mod+WheelScrollUp   { focus-column-left; }
    Mod+Ctrl+Left  { move-column-left; }
    Mod+Ctrl+Right { move-column-right; }
    Mod+F { maximize-column; }
}
NIRI

# FASTFETCH
sudo mkdir -p /etc/skel/Settings/Config/fastfetch 
cat<<FETCH > /etc/skel/Settings/Config/fastfetch/config.jsonc
{
    "$schema": "https://github.com/fastfetch-cli/fastfetch/raw/dev/doc/json_schema.json",
    "logo": {
        "type": "none"
    },
    "display": {
        "separator": " ➜  ",
    },
    "modules": [
        {
            "type": "title",
            "key": "DS  "
        },
        {
            "type": "host",
            "key": "PC  "
        },
        {
            "type": "os",
            "key": "OS  "
        },
        {
            "type": "kernel",
            "key": "KR  "
        },
        {
            "type": "uptime",
            "key": "UP  "
        },
        "break",
        {
            "type": "cpu",
            "key": "CPU ",
        },
        {
            "type": "gpu",
            "key": "GPU ",
            "format": "{2}" 
        },
        {
            "type": "memory",
            "key": "RAM ",
            "format": "{1} / {2}"
        },
        {
            "type": "disk",
            "key": "SPC ",
            "folders": "/"
        },
        {
            "type": "display",
            "key": "RES "
        },
    ]
}
FETCH

# RIO
sudo mkdir -p /etc/skel/Settings/Config/rio
cat<<RIO > /etc/skel/Settings/Config/rio/config.toml
confirm-before-quit = false
[window]
opacity = 0.5
blur = true
decorations = "Disabled" 
[renderer]
level = 1
[effects]
trail-cursor = true
RIO

# HELIX & RUST 
sudo mkdir -p /etc/skel/Settings/Config/helix /etc/skel/.cargo
cat<<HELIX > /etc/skel/Settings/Config/helix/config.toml
theme = "base16_transparent"
[editor]
end-of-line-diagnostics = "hint"
[editor.cursor-shape]
insert = "bar"
[editor.lsp]
display-inlay-hints = true
[editor.statusline]
left = ["mode", "file-name"]
right = ["position", "file-type"]
[editor.inline-diagnostics]
cursor-line = "warning"
HELIX
cat<<LANG > /etc/skel/Settings/Config/helix/languages.toml
[[language]]
name = "rust"
auto-format = true
[language-server.rust-analyzer.config.check]
command = "clippy"
LANG
cat<<RUST > /etc/skel/.cargo/config.toml
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=mold"]
RUST

# LY
mkdir -p /etc/ly
cat <<LY > /etc/ly/config.ini
animate = true
animation = matrix
cmatrix_fg = 0x008833FF
cmatrix_head_col = 0x00B38BFF
cmatrix_min_codepoint = 0x21
cmatrix_max_codepoint = 0x7B
bigclock = en
bigclock_12hr = true
battery_id = BAT0
save = true
load = true
tty = 2
wayland_cmd = niri-session
full_color = true
default_input = password
LY

# HYPRLOCK 
sudo mkdir -p /etc/skel/Settings/Config/hypr
cat<<HYPRLOCK > /etc/skel/Settings/Config/hypr/hyprlock.conf
background {
  monitor =
  path = ~/Media/Pictures/Wallpapers/af01a49d92691da098a5c3e294163237.jpg
  blur_passes = 4
  blur_size = 4
}
$base = $XDG_CONFIG_HOME/haremal-ctrl/hyprlock
$icon = $base/icon.png
source = $base/colors.conf
$music = $base/scripts/playerctlock.sh
$album = $base/scripts/hlock_mpris.sh
$battery = $base/scripts/battery.sh
$location = $base/scripts/location.sh
$weather = $base/scripts/weather.sh
source = $base/layouts/minimal.conf
HYPRLOCK

# YAZI 
sudo mkdir -p /etc/skel/Settings/Config/yazi
cat<<YAZI > /etc/skel/Settings/Config/yazi/yazi.toml
[opener]
edit = [
    { run = 'xdg-open "$@"', block = true, desc = "System Default" }
]
YAZI

# XDG PORTAL
mkdir -p /etc/skel/Settings/Config/xdg-desktop-portal/
cat <<XDG > /etc/skel/Settings/Config/xdg-desktop-portal/portals.conf
[preferred]
default=gtk
org.freedesktop.impl.portal.ScreenCast=gnome
XDG
