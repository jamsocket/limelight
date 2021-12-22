#version 300 es

in vec3 start;
in vec3 end;
in uint color;
in float width;
in vec2 line_position;
in vec2 line_edge;
uniform mat4 u_transform;

flat out uint v_color;
out vec2 v_edge;

void main() {
    vec3 line = normalize(end - start);
    // TODO: should find a perpendicular line in post-transform space instead of pre?
    vec3 perp = vec3(line.y, -line.x, 0.);

    v_edge = line_edge;
    vec3 pos = (line_position.x * end) + ((1.-line_position.x) * start) + perp * width * line_position.y;

    gl_Position = vec4(pos, 1.0) * u_transform;

    v_color = color;
}