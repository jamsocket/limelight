#version 300 es

precision highp float;

uniform float u_time;
uniform vec2 u_pos;
in vec2 v_pos;
out vec4 color;

void main() {
  if (length(u_pos - v_pos) < (sin(u_time) + 1.) / 10.) {
    color = vec4(0.8, u_pos, 1.0);
  } else {
    color = vec4(u_pos, 0.5, 1.0);
  }  
}