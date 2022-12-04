#version 300 es
precision highp float;
precision highp int;
out vec4 fragColor;
uniform float u_time;
uniform vec2 u_resolution;

const uint UINT_MAX = 0xffffffffu;
uint k = 0x456789abu;  // large number

uint u_hash11(uint n) {
    n ^= (n << 1);  // shift L -> XOR
    n ^= (n >> 1);  // shift R -> XOR
    n *= k;
    n ^= (n << 1);

    return n * k;
}

float hash11(float p) {
    uint n = floatBitsToUint(p);  // built-in

    return float(u_hash11(n)) / float(UINT_MAX);  // normalize
}

void main() {
    vec2 uv = gl_FragCoord.xy / u_resolution.xy;  // use uv for avoid elimination
    float time = floor(15.0 * u_time);  // 60 -> 15 count per sec
    vec2 pos = uv + gl_FragCoord.xy + time;  // shift frag coordinates

    fragColor.rgb = vec3(hash11(pos.x));
    fragColor.a = 1.0;
}
