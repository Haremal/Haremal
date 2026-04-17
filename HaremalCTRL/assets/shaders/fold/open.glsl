vec4 door_rise(vec3 coords_geo, vec3 size_geo) {
    float progress = niri_clamped_progress;
    float tilt = (1.0 - progress) * 1.57079632;
    vec2 coords = coords_geo.xy * size_geo.xy;
    coords.y = size_geo.y - coords.y;
    float dist_from_pivot = coords.y;
    float z_offset = -dist_from_pivot * sin(tilt);
    float y_compressed = dist_from_pivot * cos(tilt);
    float perspective = 600.0;
    float perspective_scale = perspective / (perspective + z_offset);
    coords.x = (coords.x - size_geo.x * 0.5) * perspective_scale + size_geo.x * 0.5;
    coords.y = y_compressed * perspective_scale;
    coords.y = size_geo.y - coords.y;
    coords_geo = vec3(coords / size_geo.xy, 1.0);
    vec3 coords_tex = niri_geo_to_tex * coords_geo;
    vec4 color = texture2D(niri_tex, coords_tex.st);
    float brightness = 0.4 + 0.6 * progress;
    color.rgb *= brightness;
    return color * progress;
}
vec4 open_color(vec3 coords_geo, vec3 size_geo) {
    return door_rise(coords_geo, size_geo);
}
