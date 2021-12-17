#version 300 es

in vec3 start;
in vec3 end;
in uint color;
in float width;
in uint index;
uniform mat4 u_transform;

flat out uint v_color;
out vec2 v_edge;

void main() {
    vec3 line = normalize(end - start);
    // TODO: should find a perpendicular line in post-transform space instead of pre?
    vec3 perp = vec3(line.y, -line.x, 0.);

    vec3 c1 = start - perp * width;
    vec3 c2 = start + perp * width;
    vec3 c3 = end - perp * width;
    vec3 c4 = end + perp * width;

    switch (index) {
        case 0u:
        gl_Position = vec4(c1, 1.);
        v_edge = vec2(0., 0.);
        break;
        case 1u:
        gl_Position = vec4(c2, 1.);
        v_edge = vec2(0., 1.);
        break;
        case 2u:
        gl_Position = vec4(c3, 1.);
        v_edge = vec2(1., 0.);
        break;
        case 3u:
        gl_Position = vec4(c4, 1.);
        v_edge = vec2(0., 0.);
    }

    gl_Position = gl_Position * u_transform;

    v_color = color;
}