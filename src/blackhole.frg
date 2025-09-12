#version 100
precision mediump float;

uniform sampler2D _ScreenTexture;
uniform vec2 iResolution;
uniform vec2 iBhPos;
uniform float iTime;
uniform float iBhDist;

void main() {
    float bh_radius = 0.5 / iBhDist;

    vec2 uv = gl_FragCoord.xy / iResolution;
    vec2 relative_uv = uv - iBhPos;
    vec2 ratio = vec2(iResolution.y / iResolution.x, 1.0);
    float dist_to_bh_center = length(relative_uv / ratio);

    vec3 col = vec3(0.0);

    if (dist_to_bh_center > bh_radius) {
        float deformation = 1.0 / pow(dist_to_bh_center * sqrt(iBhDist), 2.0) * bh_radius * 2.0;
        vec2 new_uv = relative_uv * (1.0 - deformation) + iBhPos;
        col = texture2D(_ScreenTexture, new_uv).rgb;
    }

    gl_FragColor = vec4(col, 1.0);
}
