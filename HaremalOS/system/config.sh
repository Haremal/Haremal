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
             layout "pl"
        }
        numlock
    }
    touchpad { off; }
}

cursor {
    xcursor-theme "Bibata-Modern-Classic"
    xcursor-size 24
}

/-output "eDP-1" {
    mode "1920x1080@120.030"
    scale 1
    transform "normal"
    position x=1280 y=0
}


layout {
    empty-workspace-above-first
    gaps 10

    shadow {
        on // shadow
    }
    focus-ring {
        off
    }
}
window-rule {
    clip-to-geometry true
    geometry-corner-radius 12
    opacity 0.95 // opacity active
}
window-rule {
    match is-active=false
    opacity 0.70 // opacity inactive
}
animations {
    window-open {
        duration-ms 300
        curve "linear"
        custom-shader r" // open unravel
vec4 line_expand(vec3 coords_geo, vec3 size_geo) { // marked_shader
    float progress = niri_clamped_progress; // marked_shader
    float eased_progress = progress * progress * (3.0 - 2.0 * progress); // marked_shader
    float window_center_y = size_geo.y * 0.5; // marked_shader
    float pixel_y = coords_geo.y * size_geo.y; // marked_shader
    float dist_from_center = abs(pixel_y - window_center_y); // marked_shader
    float visible_radius = (size_geo.y * 0.5) * eased_progress; // marked_shader
    if (dist_from_center > visible_radius) { // marked_shader
        return vec4(0.0); // marked_shader
    } // marked_shader
    float edge_thickness = 3.0; // marked_shader
    bool at_edge = abs(dist_from_center - visible_radius) < edge_thickness; // marked_shader
    vec3 coords_tex = niri_geo_to_tex * coords_geo; // marked_shader
    vec4 color = texture2D(niri_tex, coords_tex.st); // marked_shader
    if (at_edge && eased_progress < 0.99) { // marked_shader
        color.rgb = mix(color.rgb, vec3(1.0, 1.0, 1.0), 0.8); // marked_shader
    } // marked_shader
    return color; // marked_shader
} // marked_shader
vec4 open_color(vec3 coords_geo, vec3 size_geo) { // marked_shader
    return line_expand(coords_geo, size_geo); // marked_shader
} // marked_shader
        " // end
    }

    window-close {
        duration-ms 300
        curve "linear"
        custom-shader r" // close unravel
vec4 line_collapse(vec3 coords_geo, vec3 size_geo) { // marked_shader
    float progress = niri_clamped_progress; // marked_shader
    float eased_progress = progress * progress * (3.0 - 2.0 * progress); // marked_shader
    float reversed_progress = 1.0 - eased_progress; // marked_shader
    float window_center_y = size_geo.y * 0.5; // marked_shader
    float pixel_y = coords_geo.y * size_geo.y; // marked_shader
    float dist_from_center = abs(pixel_y - window_center_y); // marked_shader
    float visible_radius = (size_geo.y * 0.5) * reversed_progress; // marked_shader
    if (dist_from_center > visible_radius) { // marked_shader
        return vec4(0.0); // marked_shader
    } // marked_shader
    float edge_thickness = 2.0; // marked_shader
    bool at_edge = abs(dist_from_center - visible_radius) < edge_thickness; // marked_shader
    vec3 coords_tex = niri_geo_to_tex * coords_geo; // marked_shader
    vec4 color = texture2D(niri_tex, coords_tex.st); // marked_shader
    if (at_edge && reversed_progress > 0.01) { // marked_shader
        color.rgb = mix(color.rgb, vec3(1.0, 1.0, 1.0), 0.8); // marked_shader
    } // marked_shader
    return color; // marked_shader
} // marked_shader
vec4 close_color(vec3 coords_geo, vec3 size_geo) { // marked_shader
    return line_collapse(coords_geo, size_geo); // marked_shader
} // marked_shader
        " // end
    }
}

// startups
spawn-at-startup "hypridle"
spawn-at-startup "awww-daemon"
spawn-at-startup "dbus-update-activation-environment" "--systemd" "WAYLAND_DISPLAY" "XDG_CURRENT_DESKTOP=niri"
spawn-at-startup "/usr/lib/hyprpolkitagent"
spawn-at-startup "xwayland-satellite"

binds {
    Mod+Space { spawn "rio"; } // keybind
    XF86AudioRaiseVolume { spawn-sh "wpctl set-volume @DEFAULT_AUDIO_SINK@ 0.1+ -l 1.0"; } // keybind
    XF86AudioLowerVolume { spawn-sh "wpctl set-volume @DEFAULT_AUDIO_SINK@ 0.1-"; } // keybind
    XF86AudioMute { spawn-sh "wpctl set-mute @DEFAULT_AUDIO_SINK@ toggle"; } // keybind
    XF86AudioMicMute { spawn-sh "wpctl set-mute @DEFAULT_AUDIO_SOURCE@ toggle"; } // keybind
    XF86AudioPlay { spawn-sh "playerctl play-pause"; } // keybind
    XF86AudioStop { spawn-sh "playerctl stop"; } // keybind
    XF86AudioPrev { spawn-sh "playerctl previous"; } // keybind
    XF86AudioNext { spawn-sh "playerctl next"; } // keybind
    XF86MonBrightnessUp { spawn "brightnessctl" "--class=backlight" "set" "+10%"; } // keybind
    XF86MonBrightnessDown { spawn "brightnessctl" "--class=backlight" "set" "10%-"; } // keybind
    Mod+O { toggle-overview; } // keybind
    Mod+Q { close-window; } // keybind
    Mod+Down  { focus-window-down; } // keybind
    Mod+Up    { focus-window-up; } // keybind
    Mod+Right { focus-column-right; } // keybind
    Mod+Left { focus-column-left; } // keybind
    Mod+Ctrl+Left  { move-column-left; } // keybind
    Mod+Ctrl+Down  { move-window-down; } // keybind
    Mod+Ctrl+Up    { move-window-up; } // keybind
    Mod+Ctrl+Right { move-column-right; } // keybind
    Mod+Ctrl+H     { move-column-left; } // keybind
    Mod+Ctrl+J     { move-window-down; } // keybind
    Mod+Ctrl+K     { move-window-up; } // keybind
    Mod+Ctrl+L     { move-column-right; } // keybind
    Mod+Shift+Left  { focus-monitor-left; } // keybind
    Mod+Shift+Down  { focus-monitor-down; } // keybind
    Mod+Shift+Up    { focus-monitor-up; } // keybind
    Mod+Shift+Right { focus-monitor-right; } // keybind
    Mod+Shift+Ctrl+Left  { move-column-to-monitor-left; } // keybind
    Mod+Shift+Ctrl+Down  { move-column-to-monitor-down; } // keybind
    Mod+Shift+Ctrl+Up    { move-column-to-monitor-up; } // keybind
    Mod+Shift+Ctrl+Right { move-column-to-monitor-right; } // keybind
    Mod+Ctrl+Page_Down { move-column-to-workspace-down; } // keybind
    Mod+Ctrl+Page_Up   { move-column-to-workspace-up; } // keybind
    Mod+Shift+Page_Down { move-workspace-down; } // keybind
    Mod+Shift+Page_Up   { move-workspace-up; } // keybind
    Mod+WheelScrollDown { focus-workspace-down; } // keybind
    Mod+WheelScrollUp   { focus-workspace-up; } // keybind
    Mod+Page_Down { focus-workspace-down; } // keybind
    Mod+Page_Up { focus-workspace-up; } // keybind
    Mod+BracketLeft  { consume-or-expel-window-left; } // keybind
    Mod+BracketRight { consume-or-expel-window-right; } // keybind
    Mod+Comma  { consume-window-into-column; } // keybind
    Mod+F { maximize-column; } // keybind
    Mod+Shift+F { fullscreen-window; } // keybind
    Mod+C { center-column; } // keybind
    Mod+Minus { set-column-width "-10%"; } // keybind
    Mod+Equal { set-column-width "+10%"; } // keybind
    Mod+Shift+Minus { set-window-height "-10%"; } // keybind
    Mod+Shift+Equal { set-window-height "+10%"; } // keybind
    Mod+V       { toggle-window-floating; } // keybind
    Mod+Shift+V { switch-focus-between-floating-and-tiling; } // keybind
    Print { screenshot; } // keybind
    Mod+Shift+P { power-off-monitors; } // keybind
}

screenshot-path "~/Media/Pictures/Screenshots/Screenshot from %Y-%m-%d %H-%M-%S.png"

gestures {
    hot-corners {
        off
    }
}
hotkey-overlay {
    skip-at-startup
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
