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
  // uniform float u_time;

  void main() {
    vec2 pos = gl_FragCoord.xy / u_resolution.xy;
    vec3 RED = vec3(1.0, 0.0, 0.0);
    vec3 BLUE = vec3(0.0, 0.0, 1.0);
    vec3 col = mix(RED, BLUE, pos.x);

    fragColor = vec4(col, 1.0);
    // fragColor = vec4(abs(sin(u_time)), 0.0, 0.0, 1.0);
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
