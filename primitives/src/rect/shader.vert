#version 300 es

in vec2 upper_left;
in vec2 lower_right;
in uint color;
in vec2 rect_position;

flat out uint v_color;
uniform mat4 u_transform;

void main() {
    gl_Position = vec4(
        upper_left.x * (1. - rect_position.x) + lower_right.x * rect_position.x,
        upper_left.y * (1. - rect_position.y) + lower_right.y * rect_position.y,
        0.,
        1.
    );

    gl_Position = gl_Position * u_transform;

    v_color = color;
}