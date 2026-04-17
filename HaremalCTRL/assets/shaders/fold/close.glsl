vec4 bob_and_slide(vec3 coords_geo, vec3 size_geo) {
    float progress = niri_clamped_progress;
    float y_offset = 0.0;
    if (progress < 0.25) {
        float t = progress / 0.25;
        y_offset = -40.0 * (1.0 - 4.0 * (t - 0.5) * (t - 0.5));
    }
    else {
        float slide_progress = (progress - 0.25) / 0.75;
        y_offset = -slide_progress * (size_geo.y + 100.0);
    }
    vec2 coords = coords_geo.xy * size_geo.xy;
    coords.y = coords.y + y_offset;
    coords_geo = vec3(coords / size_geo.xy, 1.0);
    vec3 coords_tex = niri_geo_to_tex * coords_geo;
    vec4 color = texture2D(niri_tex, coords_tex.st);
    return color;
}
vec4 close_color(vec3 coords_geo, vec3 size_geo) {
    return bob_and_slide(coords_geo, size_geo);
}
