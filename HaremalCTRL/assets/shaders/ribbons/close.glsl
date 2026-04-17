vec4 close_color(vec3 coords_geo, vec3 size_geo) {
    float progress = niri_clamped_progress;
    float random_angle = niri_random_seed * 6.28318;
    vec2 coords = coords_geo.xy - 0.5;
    float cos_a = cos(random_angle);
    float sin_a = sin(random_angle);
    vec2 rotated = vec2(
        coords.x * cos_a - coords.y * sin_a,
        coords.x * sin_a + coords.y * cos_a
    );
    float y_pos = rotated.y + 0.5;
    float ribbon_count = 20.0;
    float ribbon_index = floor(y_pos * ribbon_count);
    float direction = mod(ribbon_index, 2.0) == 0.0 ? -1.0 : 1.0;
    float delay = ribbon_index / ribbon_count * 0.5;
    float ribbon_progress = clamp((progress - delay) / (1.0 - delay), 0.0, 1.0);
    rotated.x += ribbon_progress * direction * 2.0;
    coords = vec2(
        rotated.x * cos_a + rotated.y * sin_a,
        -rotated.x * sin_a + rotated.y * cos_a
    );
    coords += 0.5;
    vec3 coords_tex = niri_geo_to_tex * vec3(coords.x, coords.y, 1.0);
    vec4 color = texture2D(niri_tex, coords_tex.xy);
    if (coords.x < 0.0 || coords.x > 1.0) {
        return vec4(0.0);
    }
    return color;
}
