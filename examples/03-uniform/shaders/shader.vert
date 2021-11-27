#version 300 es

const float PI = 3.141592653589793238;
in vec2 position;
uniform float u_rotate;
uniform vec2 u_scale;
out vec2 v_position;

void main() {
  float angle = (float(gl_VertexID) + 3./4.) * 2. * PI / 3. + u_rotate;
  v_position = vec2(cos(angle) / 1.2, sin(angle) / 1.2) * u_scale;
  gl_Position = vec4(v_position, 0., 1.);
}