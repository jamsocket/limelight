#version 300 es

uniform float u_time;

in vec2 position;

// From: https://thebookofshaders.com/08/
mat2 rotate2d(float _angle){
    return mat2(cos(_angle),-sin(_angle),
                sin(_angle),cos(_angle));
}
 
void main() {
  gl_Position = vec4(position * rotate2d(u_time), 0., 1.);
}