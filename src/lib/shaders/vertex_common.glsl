#version 300 es
// The individual position vertex
in vec2 position;
  
void main() {
    // the gl_Position is the final position in clip space 
    // after the vertex shader modifies it
    gl_Position = vec4(position, 0.0, 1.0);
}
