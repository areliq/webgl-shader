<svelte:head>
  <title>Test It Works</title>
  <meta name="description" content="WebGL Shader App" />
</svelte:head>

<canvas id="glcanvas" />

<script lang="ts">
import { onMount } from 'svelte';
import { Whiteboard } from "$lib/client/whiteboard2d"

onMount(async () => {
  const c = new Whiteboard("#glcanvas");

  const renderLoop: FrameRequestCallback = (timestamp) => {
    c.tick(timestamp / 1000);
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