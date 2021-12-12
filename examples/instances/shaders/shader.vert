#version 300 es

const float PI = 3.141592653589793238;
in int instance_index;
in int vertex_index;
out vec2 v_position;
out vec2 m_position;

void main() {
  float angle = (float(instance_index) + (3./4.)) * 2. * PI / 20.;
  v_position = vec2(cos(angle) / 2., sin(angle) / 2.);

  float angle2 = (float(vertex_index) + (3./4.)) * 2. * PI / 4.;
  v_position += vec2(cos(angle2) / 10., sin(angle2) / 10.);

  gl_PointSize = 10.0;
  gl_Position = vec4(v_position, 0., 1.);
  m_position = vec2(float(instance_index) / 20., float(vertex_index) / 4.);
}