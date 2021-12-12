#version 300 es

precision highp float;

//in vec4 v_color;
in vec2 v_edge;

out vec4 f_color;

void main() {
    float dx = fwidth(v_edge.x);
    float dy = fwidth(v_edge.y);

    float xcov = min(clamp(0., 1., v_edge.x / dx), clamp(0., 1., (1. - v_edge.x) / dx));
    float ycov = min(clamp(0., 1., v_edge.y / dy), clamp(0., 1., (1. - v_edge.y) / dy));
    float alpha = xcov * ycov;

    f_color = vec4(vec3(0.,0.4,0.6) * alpha, alpha);
}