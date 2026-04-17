float ease_curve(float x) {
    return x < 0.5 ? 4.0*x*x*x : 1.0 - pow(-2.0*x + 2.0, 3.0)/2.0;
}
vec4 close_color(vec3 coords_geo, vec3 size_geo) {
    float t = niri_clamped_progress;
    float prog = ease_curve(t);
    int corner = 0; 
    vec2 start;
    if (corner == 0) start = vec2(0.0,0.0);
    else if (corner == 1) start = vec2(1.0,0.0);
    else if (corner == 2) start = vec2(0.0,1.0);
    else start = vec2(1.0,1.0);
    vec2 p = coords_geo.xy;
    float dist = dot(p - start, vec2(1.0,1.0));
    float max_diag = 2.0; // max of vec2(1,1)
    float norm_dist = dist / max_diag;
    if (norm_dist <= prog) {
        return vec4(0.0);
    }
    vec3 coords_tex = niri_geo_to_tex * coords_geo;
    vec4 col = texture2D(niri_tex, coords_tex.xy);
    return col;
}
