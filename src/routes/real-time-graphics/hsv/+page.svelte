  <script lang="ts">
    import vertexShader from '$lib/shaders/vertex_common.glsl';
    import fragmentShader from '$lib/shaders/1_4_hsv.glsl';
    import { onMount } from 'svelte';
    import init, { GlBox } from '$lib/wasm/pkg';

    onMount(async () => {
      await init();

      const square = GlBox.new("canvas", false, vertexShader, fragmentShader);

      // const renderLoop: FrameRequestCallback = (timestamp) => {
      //   square.tick(timestamp / 1000);
        square.draw();

      //   requestAnimationFrame(renderLoop);
      // };

      // requestAnimationFrame(renderLoop);
    })
  </script>
  
  <svelte:head>
    <title>Polar-Rot</title>
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
