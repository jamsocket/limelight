#version 300 es

precision highp float;

flat in uint v_color;
out vec4 f_color;

void main() {
    f_color = vec4(
        float((v_color & 0x000000FFu)) / 255.,
        float((v_color & 0x0000FF00u) >> 8) / 255.,
        float((v_color & 0x00FF0000u) >> 16) / 255.,
        float((v_color & 0xFF000000u) >> 24) / 255.);
}