<script lang="ts">
  import { onMount } from 'svelte';
  import init, { RotatingCube } from '$lib/wasm/pkg';

  onMount(async () => {
    await init();

    const cube = RotatingCube.new('canvas');
    const renderLoop: FrameRequestCallback = (timestamp) => {
      const delta = timestamp / 1000;
      cube.tick(delta);
      cube.draw();

      requestAnimationFrame(renderLoop);
    };

    requestAnimationFrame(renderLoop);
  });
</script>

<svelte:head>
  <title>WebGL</title>
  <meta name="description" content="WebGL Shader App" />
</svelte:head>

<canvas id="canvas" />

<style>
  canvas {
    width: 100vw;
    height: 100vh;
    display: block;
  }
</style>
