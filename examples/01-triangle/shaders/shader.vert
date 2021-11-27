#version 300 es

const float PI = 3.141592653589793238;
in vec2 position;
out vec2 v_position;

void main() {
  float angle = (float(gl_VertexID) + (3./4.)) * 2. * PI / 3.;
  v_position = vec2(cos(angle) / 1.2, sin(angle) / 1.2);
  gl_Position = vec4(v_position, 0., 1.);
}