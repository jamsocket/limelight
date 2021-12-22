#version 300 es

in vec2 position;
in uint color;
in float radius;
in vec2 relative_position;

flat out uint v_color;
out vec2 v_edge;
out vec2 v_coord;

uniform mat4 u_transform;

void main() {
    // switch (gl_VertexID) {
    //     case 0:
    //         gl_Position = vec4(position.x - radius, position.y - radius, 0., 1.);
    //         v_coord = vec2(-1., -1.);
    //         break;
    //     case 1:
    //         gl_Position = vec4(position.x + radius, position.y - radius, 0., 1.);
    //         v_coord = vec2(1., -1.);
    //         break;
    //     case 2:
    //         gl_Position = vec4(position.x - radius, position.y + radius, 0., 1.);
    //         v_coord = vec2(-1., 1.);
    //         break;
    //     case 3:
    //         gl_Position = vec4(position.x + radius, position.y + radius, 0., 1.);
    //         v_coord = vec2(1., 1.);
    // }

    v_coord = relative_position;
    gl_Position = vec4(position + radius * relative_position, 0., 1.) * u_transform;

    v_color = color;
}
	