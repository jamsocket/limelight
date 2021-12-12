#version 300 es

in float location;
in uint orientation;
in uint color;

flat out uint v_color;
uniform mat4 u_transform;

const float THICKNESS = 0.002;

void main() {
    vec4 scaled = u_transform * vec4(location, location, 0.0, 1.0);

    if (orientation == 0u) {
        /* Horizontal */
        switch (gl_VertexID) {
            case 0:
                gl_Position = vec4(-1., scaled.y - THICKNESS, 0., 1.);
                break;
            case 1:
                gl_Position = vec4(1., scaled.y - THICKNESS, 0., 1.);
                break;
            case 2:
                gl_Position = vec4(-1., scaled.y + THICKNESS, 0., 1.);
                break;
            case 3:
                gl_Position = vec4(1., scaled.y + THICKNESS, 0., 1.);
        }
    } else {
        /* Vertical */
        switch (gl_VertexID) {
            case 0:
                gl_Position = vec4(scaled.x - THICKNESS, 1., 0., 1.);
                break;
            case 1:
                gl_Position = vec4(scaled.x - THICKNESS, -1., 0., 1.);
                break;
            case 2:
                gl_Position = vec4(scaled.x + THICKNESS, 1., 0., 1.);
                break;
            case 3:
                gl_Position = vec4(scaled.x + THICKNESS, -1., 0., 1.);
        }
    }

    v_color = color;
}