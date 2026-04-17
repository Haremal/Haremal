            float hash(vec2 p) { return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453); }
            float noise(vec2 p) {
                vec2 i = floor(p); vec2 f = fract(p);
                f = f * f * (3.0 - 2.0 * f);
                return mix(mix(hash(i), hash(i + vec2(1.0, 0.0)), f.x),
                           mix(hash(i + vec2(0.0, 1.0)), hash(i + vec2(1.0, 1.0)), f.x), f.y);
            }
            vec3 get_ember_colors(float seed, out vec3 inner, out vec3 outer) {
                if (seed < 0.125) { inner = vec3(1.0, 0.3, 0.0); outer = vec3(1.0, 0.8, 0.2); }       // orange
                else if (seed < 0.250) { inner = vec3(0.2, 0.4, 1.0); outer = vec3(0.5, 0.8, 1.0); } // blue
                else if (seed < 0.375) { inner = vec3(0.6, 0.1, 0.9); outer = vec3(0.9, 0.5, 1.0); } // purple
                else if (seed < 0.500) { inner = vec3(0.1, 0.8, 0.2); outer = vec3(0.5, 1.0, 0.3); } // green
                else if (seed < 0.625) { inner = vec3(1.0, 0.1, 0.4); outer = vec3(1.0, 0.5, 0.7); } // pink
                else if (seed < 0.750) { inner = vec3(0.0, 0.8, 0.9); outer = vec3(0.7, 1.0, 1.0); } // cyan
                else if (seed < 0.875) { inner = vec3(0.9, 0.7, 0.1); outer = vec3(1.0, 1.0, 0.8); } // gold
                else { inner = vec3(0.8, 0.2, 0.1); outer = vec3(1.0, 0.5, 0.2); }                   // red
                return inner;
            }
            vec4 open_color(vec3 coords_geo, vec3 size_geo) {
                if (coords_geo.x < 0.0 || coords_geo.x > 1.0 || coords_geo.y < 0.0 || coords_geo.y > 1.0) return vec4(0.0);
                float progress = niri_clamped_progress;
                vec2 uv = coords_geo.xy;
                vec3 coords_tex = niri_geo_to_tex * vec3(uv, 1.0);
                vec4 color = texture2D(niri_tex, coords_tex.st);
                float edge_dist = min(min(uv.x, 1.0 - uv.x), min(uv.y, 1.0 - uv.y));
                float n = noise(uv * 8.0 + niri_random_seed * 100.0) * 0.3;
                float burn_line = edge_dist + n;
                float threshold = progress * 0.8;
                vec3 ember_inner, ember_outer;
                get_ember_colors(niri_random_seed, ember_inner, ember_outer);
                if (burn_line < threshold - 0.08) return color;
                else if (burn_line < threshold) {
                    vec3 ember = mix(ember_inner, ember_outer, (burn_line - threshold + 0.08) / 0.08);
                    return vec4(mix(ember, color.rgb, 0.3), color.a);
                } else return vec4(0.0);
            }
