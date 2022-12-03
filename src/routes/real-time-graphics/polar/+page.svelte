  <script>
    import vertexShader from '$lib/shaders/vertex_common.glsl';
    import fragmentShader from '$lib/shaders/1_4_polar.glsl';
    import { onMount } from 'svelte';
    import init, { GlBox } from '$lib/wasm/pkg';

    onMount(async () => {
      await init();
      
      const square = GlBox.new("canvas", false, vertexShader, fragmentShader);
  
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
    /* .glsl {
      white-space: pre-line;
    } */
  </style>
  
  <canvas id="canvas"></canvas>
  <!-- <p class="glsl">{vertexShader}</p> -->
