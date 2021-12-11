#version 300 es

const float PI = 3.141592653589793238;
in int instance_index;
in int vertex_index;
out vec2 v_position;
out vec2 m_position;

void main() {
  float angle = (float(vertex_index) + (3./4.)) * 2. * PI / 10.;
  v_position = vec2(cos(angle) / 5., sin(angle) / 5.);

  float angle2 = (float(instance_index) + (3./4.)) * 2. * PI / 3.;
  v_position += vec2(cos(angle2) / 50., sin(angle2) / 50.);

  gl_PointSize = 10.0;
  gl_Position = vec4(v_position, 0., 1.);
  m_position = vec2(float(vertex_index) / 10., float(instance_index) / 100000000.);
}