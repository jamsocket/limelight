#version 300 es

in vec2 start;
in vec2 end;
in uint color;
in float width;
in vec2 line_position;
in vec2 line_edge;
uniform mat4 u_transform;

flat out uint v_color;
out vec2 v_edge;

void main() {
    vec2 line = normalize(end - start);
    vec2 perp = vec2(line.y, -line.x);

    v_edge = line_edge;
    vec2 pos = (line_position.x * end) + ((1.-line_position.x) * start) + perp * width * line_position.y;

    gl_Position = vec4(pos, 0., 1.) * u_transform;
    v_edge = line_edge;

    v_color = color;
}