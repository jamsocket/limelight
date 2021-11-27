#version 300 es

in vec2 position;
uniform mat4 u_transform;
 
void main() {
  gl_Position = vec4(position * 0.5, 0., 1.) * u_transform;
}