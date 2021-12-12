#version 300 es

in vec2 start;
in vec2 end;
in uint color;
in float width;
uniform mat4 u_transform;

flat out uint v_color;
out vec2 v_edge;

void main() {
    vec2 line = normalize(end - start);
    vec2 perp = vec2(line.y, -line.x);

    vec2 c1 = start - perp * width;
    vec2 c2 = start + perp * width;
    vec2 c3 = end - perp * width;
    vec2 c4 = end + perp * width;

    switch (gl_VertexID) {
        case 0:
        gl_Position = vec4(c1, 0., 1.);
        v_edge = vec2(0., 0.);
        break;
        case 1:
        gl_Position = vec4(c2, 0., 1.);
        v_edge = vec2(0., 1.);
        break;
        case 2:
        gl_Position = vec4(c3, 0., 1.);
        v_edge = vec2(1., 0.);
        break;
        case 3:
        gl_Position = vec4(c4, 0., 1.);
        v_edge = vec2(0., 0.);
    }

    gl_Position = u_transform * gl_Position;

    v_color = color;
}