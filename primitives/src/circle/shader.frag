#version 300 es

precision highp float;

flat in uint v_color;
in vec2 v_coord;

out vec4 f_color;

void main() {
    float r = dot(v_coord, v_coord);
    float delta = fwidth(r);

    float alpha = 1.0 - smoothstep(1.0 - delta*2., 1.0, r);

    if (alpha < 0.01) {
        discard;
    }

    f_color = vec4(
        float((v_color & 0x000000FFu)) / 255.,
        float((v_color & 0x0000FF00u) >> 8) / 255.,
        float((v_color & 0x00FF0000u) >> 16) / 255.,
        float((v_color & 0xFF000000u) >> 24) / 255.) * alpha;
}