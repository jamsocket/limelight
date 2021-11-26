#version 300 es

in vec4 position;
out vec2 v_pos;

const vec2 positions[3] = vec2[3](
  vec2(-1., 1.),
  vec2(3., 1.),
  vec2(-1., -3.)
);

void main() {
  v_pos = positions[gl_VertexID];
  gl_Position = vec4(v_pos, 0., 1.);
}
