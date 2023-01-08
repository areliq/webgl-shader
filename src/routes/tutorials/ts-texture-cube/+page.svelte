<script lang="ts">
  import { onMount } from 'svelte';
  import { TextureCube } from '$lib/client/cube-texture';
  import texture from '$lib/images/texture.png';

  onMount(async () => {
    const c = new TextureCube('#glcanvas');

    const img = new Image();
    img.src = texture;

    img.onload = () => {
      console.log('image onload');
      c.loadImageToTexture(img);
    };

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

<svelte:head>
  <title>WebGL Tutorial - Using Texture</title>
  <meta name="description" content="WebGL Shader App" />
</svelte:head>

<canvas id="glcanvas" />

<img src={texture} alt="texture" />

<style>
  canvas {
    width: 640px;
    height: 480px;
    display: block;
  }
</style>
