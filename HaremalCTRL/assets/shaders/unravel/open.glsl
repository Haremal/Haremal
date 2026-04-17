vec4 line_expand(vec3 coords_geo, vec3 size_geo) {
    float progress = niri_clamped_progress;
    float eased_progress = progress * progress * (3.0 - 2.0 * progress);
    float window_center_y = size_geo.y * 0.5;
    float pixel_y = coords_geo.y * size_geo.y;
    float dist_from_center = abs(pixel_y - window_center_y);
    float visible_radius = (size_geo.y * 0.5) * eased_progress;
    if (dist_from_center > visible_radius) {
        return vec4(0.0);
    }
    float edge_thickness = 3.0;
    bool at_edge = abs(dist_from_center - visible_radius) < edge_thickness;
    vec3 coords_tex = niri_geo_to_tex * coords_geo;
    vec4 color = texture2D(niri_tex, coords_tex.st);
    if (at_edge && eased_progress < 0.99) {
        color.rgb = mix(color.rgb, vec3(1.0, 1.0, 1.0), 0.8);
    }
    return color;
}
vec4 open_color(vec3 coords_geo, vec3 size_geo) {
    return line_expand(coords_geo, size_geo);
}
