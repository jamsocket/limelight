#version 300 es
 
precision highp float;
 
out vec4 color;
in vec2 v_position;
in vec2 m_position;

void main() {
  color = vec4(m_position.xy, 0., 1.0);
}