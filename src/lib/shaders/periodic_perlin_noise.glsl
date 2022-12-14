#version 300 es
precision highp float;
precision highp int;
out vec4 fragColor;
uniform float u_time;
uniform vec2 u_resolution;

const float PI = 3.1415926;

float atan2(float y, float x) {
    return x == 0.0 ? sign(y) * PI / 2.0 : atan(y, x);
}

vec2 xy2pol(vec2 xy) {
    return vec2(atan2(xy.x, xy.y), length(xy));
}

vec2 pol2xy(vec2 pol) {
    return pol.y * vec2(cos(pol.x), sin(pol.x));
}

uint uhash11(uint n) {
    n ^= (n << 1u);
    n ^= (n >> 1u);
    n *= 0x456789abu;
    n ^= (n << 1u);
    return n * 0x456789abu;
}

float gtable2(vec2 lattice, vec2 p) {
    uvec2 n = floatBitsToUint(lattice);
    uint idx = (uhash11(uhash11(n.x) + n.y) >> 29);
    float u = 0.92387953 * (idx < 4u ? p.x : p.y);  //0.92387953 = cos(pi/8)
    float v = 0.38268343 * (idx < 4u ? p.y : p.x);  //0.38268343 = sin(pi/8)

    return ((idx & 1u) == 0u ? u : -u) + ((idx & 2u) == 0u ? v : -v);
}

float periodicNoise21(vec2 p, float period) {
    vec2 n = floor(p);
    vec2 f = fract(p);
    float[4] v;

    for (int j = 0; j < 2; j++) {
        for (int i = 0; i < 2; i++) {
            v[i + 2 * j] = gtable2(mod(n + vec2(i, j), period), f - vec2(i, j));
        }
    }

    f = f * f * f * (10.0 - 15.0 * f + 6.0 * f * f);

    return 0.5 * mix(
        mix(v[0], v[1], f[0]), 
        mix(v[2], v[3], f[0]), 
        f[1]
    ) + 0.5;
}

float gtable3(vec3 lattice, vec3 p) {
    uvec3 n = floatBitsToUint(lattice);
    uint idx = (uhash11(uhash11(uhash11(n.x) + n.y) + n.z) >> 28);
    float u = idx < 8u ? p.x : p.y;
    float v = idx < 4u ? p.y : idx == 12u || idx == 14u ? p.x : p.z;
    return ((idx & 1u) == 0u ? u: -u) + ((idx & 2u) == 0u ? v : -v);
}

float periodicNoise31(vec3 p, float period) {
    vec3 n = floor(p);
    vec3 f = fract(p);
    float[8] v;

    for (int k = 0; k < 2; k++) {
        for (int j = 0; j < 2; j++) {
            for (int i = 0; i < 2; i++) {
                v[i + 2 * j + 4 * k] = gtable3(mod(n + vec3(i, j, k), period), f - vec3(i, j, k)) * 0.70710678;
            }
        }
    }

    f = f * f * f * (10.0 - 15.0 * f + 6.0 * f * f);

    float[2] w;    
    for (int i = 0; i < 2; i++) {
        w[i] = mix(
            mix(v[4 * i], v[4 * i + 1], f[0]), 
            mix(v[4 * i + 2], v[4 * i + 3], f[0]), 
            f[1]
        );
    }

    return 0.5 * mix(w[0], w[1], f[2]) + 0.5;
}

void main() {
    vec2 pos = gl_FragCoord.xy / u_resolution.xy;
    pos = 2.0 * pos.xy - vec2(1.0);
    pos = xy2pol(pos);
    pos = vec2(5.0 / PI, 5.0) * pos + u_time;

    fragColor = vec4(periodicNoise21(pos, 10.0));
    fragColor.a = 1.0;
}
