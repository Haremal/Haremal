float ease_curve(float x) {
    return x < 0.5 ? 4.0*x*x*x : 1.0 - pow(-2.0*x + 2.0, 3.0)/2.0;
}
vec4 open_color(vec3 coords_geo, vec3 size_geo) {
    float t = niri_clamped_progress;
    float prog = ease_curve(t);
    vec2 start = vec2(1.0, 1.0);
    vec2 p = coords_geo.xy;
    vec2 dir = vec2(-1.0, -1.0);
    float dist = dot(p - start, dir);
    float max_diag = 2.0;
    float norm_dist = dist / max_diag;
    if (norm_dist > prog) {
        return vec4(0.0);
    }
    vec3 coords_tex = niri_geo_to_tex * coords_geo;
    vec4 col = texture2D(niri_tex, coords_tex.xy);
    return col;
}
