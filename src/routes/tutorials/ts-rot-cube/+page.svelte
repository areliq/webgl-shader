<svelte:head>
  <title>WebGL Tutorial - Rotating Cube</title>
  <meta name="description" content="WebGL Shader App" />
</svelte:head>

<canvas id="glcanvas" />

<script lang="ts">
import { onMount } from 'svelte';
import { RotatingCube } from "$lib/client/cube-rotating"
// import texture from "$lib/images/texture.png"

onMount(async () => {
  const c = new RotatingCube("#glcanvas");

  // const img = new Image();
  // img.src = texture;

  // img.onload = () => {
  //   cube.loadImageToTexture(img);
  // }
  const start = Date.now() / 1000;

  const renderLoop: FrameRequestCallback = (timestamp) => {
    const delta = timestamp / 1000;

    c.tick(delta - start);
    c.draw();
    requestAnimationFrame(renderLoop);
  };

  requestAnimationFrame(renderLoop);
});

</script>

<style>
  canvas {
    width: 100vw;
    height: 100vh;
    display: block;
  }
</style>
