#version 300 es

precision highp float;

flat in uint v_color;
out vec4 f_color;

void main() {
    float alpha = float((v_color & 0xFF000000u) >> 24) / 255.;
    f_color = vec4(
        alpha * float((v_color & 0x000000FFu)) / 255.,
        alpha * float((v_color & 0x0000FF00u) >> 8) / 255.,
        alpha * float((v_color & 0x00FF0000u) >> 16) / 255.,
        alpha);
}