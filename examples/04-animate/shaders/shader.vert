#version 300 es

in vec2 position;
 
void main() {
  gl_Position = vec4(position * 0.5, 0., 1.);
}