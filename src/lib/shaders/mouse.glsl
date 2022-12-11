#version 300 es
precision highp float;

out vec4 fragColor;

uniform float u_time;
uniform vec2 u_resolution;
uniform vec2 u_mouse_pos;


void main() {
    vec2 pos = gl_FragCoord.xy / u_resolution.xy;
    vec2 mouse;
    mouse.x = u_mouse_pos.x / u_resolution.x;
    mouse.y = (u_resolution.y - u_mouse_pos.y) / u_resolution.y; //0.5;

    float r = mouse.x >= pos.x ? 1.0 : 0.0;
    float g = mouse.y >= pos.y ? 1.0 : 0.0;

    fragColor = vec4(r, g, abs(sin(u_time)), 1.0);
}
