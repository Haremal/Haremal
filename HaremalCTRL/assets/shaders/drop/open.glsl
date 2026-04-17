vec4 fall_from_top(vec3 coords_geo, vec3 size_geo) {
    float progress = niri_clamped_progress * niri_clamped_progress;
    vec2 coords = (coords_geo.xy - vec2(0.5, 0.0)) * size_geo.xy;
    coords.y += (1.0 - progress) * 1440.0;
    float max_angle = mix(-0.5, 0.5, floor(niri_random_seed * 4.0) / 3.0);
    float angle = (1.0 - progress) * max_angle;
    mat2 rotate = mat2(cos(angle), -sin(angle), sin(angle), cos(angle));
    coords = rotate * coords;
    coords_geo = vec3(coords / size_geo.xy + vec2(0.5, 0.0), 1.0);
    vec3 coords_tex = niri_geo_to_tex * coords_geo;
    return texture2D(niri_tex, coords_tex.st);
}
vec4 open_color(vec3 coords_geo, vec3 size_geo) {
    return fall_from_top(coords_geo, size_geo);
}
