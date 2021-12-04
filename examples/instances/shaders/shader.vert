#version 300 es

const float PI = 3.141592653589793238;
in vec2 position;
out vec2 v_position;
out vec2 m_position;

void main() {
  float angle = (float(gl_VertexID) + (3./4.)) * 2. * PI / 10.;
  v_position = vec2(cos(angle) / 5., sin(angle) / 5.);

  // float angle2 = (float(gl_InstanceID) + (3./4.)) * 2. * PI / 3.;
  // v_position += vec2(cos(angle2) / 3., sin(angle2) / 3.);

  gl_PointSize = 10.0;
  gl_Position = vec4(v_position, 0., 1.);
  //m_position = vec2(float(gl_VertexID) / 256., float(gl_InstanceID) / 256.);
  m_position = vec2(float(gl_VertexID) / 10., float(gl_InstanceID) / 100000000.);
}