#version 300 es

in float location;
in uint orientation;
in uint color;
in vec2 relative_position;

flat out uint v_color;
uniform mat4 u_transform;

const float THICKNESS = 0.002;

void main() {
    vec4 scaled = vec4(location, location, 0.0, 1.0) * u_transform;

    if (orientation == 0u) {
        /* Horizontal */
        gl_Position = vec4(
            relative_position.x,
            scaled.y - relative_position.y * THICKNESS,
            0.,
            1.
        );
    } else {
        /* Vertical */
        gl_Position = vec4(
            scaled.x - relative_position.x * THICKNESS,
            relative_position.y,
            0.,
            1.
        );
    }

    v_color = color;
}