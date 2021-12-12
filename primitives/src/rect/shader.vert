#version 300 es

in vec2 upper_left;
in vec2 lower_right;
in uint color;

flat out uint v_color;
uniform mat4 u_transform;

void main() {
    switch (gl_VertexID) {
        case 0:
            gl_Position = vec4(upper_left, 0., 1.);
            break;
        case 1:
            gl_Position = vec4(upper_left.x, lower_right.y, 0., 1.);
            break;
        case 2:
            gl_Position = vec4(lower_right.x, upper_left.y, 0., 1.);
            break;
        case 3:
            gl_Position = vec4(lower_right, 0., 1.);
    }

    gl_Position = u_transform * gl_Position;

    v_color = color;
}