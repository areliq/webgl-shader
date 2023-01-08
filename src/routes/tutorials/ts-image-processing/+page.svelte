<script lang="ts">
  import { TextureBoard } from '$lib/client/image-processing';
  import texture from '$lib/images/texture.png';
  import { onMount } from 'svelte';

  onMount(async () => {
    const c = new TextureBoard('#glcanvas');

    const img = new Image();
    img.src = texture;

    img.onload = () => {
      setTimeout(() => {
        c.loadImage(img);
        c.draw();
      }, 2000);
    };

    c.draw();
  });
</script>

<svelte:head>
  <title>Test It Works</title>
  <meta name="description" content="WebGL Shader App" />
</svelte:head>

<canvas id="glcanvas" />

<style>
  canvas {
    width: 100vw;
    height: 100vh;
    display: block;
  }
</style>
