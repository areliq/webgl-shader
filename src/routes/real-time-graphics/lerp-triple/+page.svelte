<script>
  import { onMount } from 'svelte';
  import init, { GlBox } from '$lib/wasm/pkg';

  const vertex_shader = "#version 300 es" + `
  // The individual position vertex
  in vec2 position;

  void main() {
    // the gl_Position is the final position in clip space 
    // after the vertex shader modifies it
    gl_Position = vec4(position, 0.0, 1.0);
  }`;

  const fragment_shader = "#version 300 es" + `
  precision mediump float;
  out vec4 fragColor;
  uniform vec2 u_resolution;

  void main() {
    vec2 pos = gl_FragCoord.xy / u_resolution.xy;
    vec3[3] col3 = vec3[](
      vec3(1.0, 0.0, 0.0),
      vec3(0.0, 0.0, 1.0),
      vec3(0.0, 1.0, 0.0)
    );
    pos.x *= 2.0;
    int idx = int(pos.x);
    
    vec3 col = mix(col3[idx], col3[idx + 1], fract(pos.x));

    fragColor = vec4(col, 1.0);
  }`;
  
  onMount(async () => {
    await init();
    
    const square = GlBox.new("canvas", false, vertex_shader, fragment_shader);

    square.draw();
  })
</script>

<svelte:head>
  <title>Hello World</title>
  <meta name="description" content="WebGL Shader App" />
</svelte:head>

<style>
  canvas {
    width: 100vw;
    height: 100vh;
    display: block;
  }
</style>

<canvas id="canvas"></canvas>
