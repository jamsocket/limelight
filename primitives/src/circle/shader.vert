#version 300 es

in vec2 position;
in uint color;
in float radius;

flat out uint v_color;
out vec2 v_edge;
out vec2 v_coord;

uniform mat4 u_transform;

void main() {
    switch (gl_VertexID) {
        case 0:
            gl_Position = vec4(position.x - radius, position.y - radius, 0., 1.);
            v_coord = vec2(-1., -1.);
            break;
        case 1:
            gl_Position = vec4(position.x + radius, position.y - radius, 0., 1.);
            v_coord = vec2(1., -1.);
            break;
        case 2:
            gl_Position = vec4(position.x - radius, position.y + radius, 0., 1.);
            v_coord = vec2(-1., 1.);
            break;
        case 3:
            gl_Position = vec4(position.x + radius, position.y + radius, 0., 1.);
            v_coord = vec2(1., 1.);
    }

    gl_Position = u_transform * gl_Position;

    v_color = color;
}
	