#version 300 es

in vec4 position;
const vec2 positions[3] = vec2[3](
  vec2(-0.5, 0.5),
  vec2(0.5, -0.5),
  vec2(0.5, 0.5)
);

void main() {
  gl_Position = vec4(positions[gl_VertexID], 0., 1.);
}