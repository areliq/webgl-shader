#version 300 es
precision highp float;

out vec4 fragColor;

uniform float u_time;
uniform vec2 u_resolution;


void main() {
    vec2 pos = gl_FragCoord.xy / u_resolution.xy;

    fragColor = vec4(abs(sin(u_time)), pos, 1.0);
}
