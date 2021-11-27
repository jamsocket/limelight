#version 300 es
 
precision highp float;
 
out vec4 color;
in vec2 v_position;
 
void main() {
  color = vec4(v_position.xy / 1.5 + 0.5, 0.3, 1.0);
}