#version 300 es

precision highp float;

flat in uint v_color;
in vec2 v_edge;

out vec4 f_color;

void main() {
    float dx = fwidth(v_edge.x);
    float dy = fwidth(v_edge.y);

    float xcov = min(clamp(0., 1., v_edge.x / dx), clamp(0., 1., (1. - v_edge.x) / dx));
    float ycov = min(clamp(0., 1., v_edge.y / dy), clamp(0., 1., (1. - v_edge.y) / dy));
    float alpha = xcov * ycov;

    f_color = vec4(
        float((v_color & 0x000000FFu)) / 255.,
        float((v_color & 0x0000FF00u) >> 8) / 255.,
        float((v_color & 0x00FF0000u) >> 16) / 255.,
        float((v_color & 0xFF000000u) >> 24) / 255.);
}