<script>
    import { onMount } from 'svelte';
    import init, { HelloBoard } from '$lib/wasm/pkg';
  
    onMount(async () => {
      await init();
    
      const square = HelloBoard.new("canvas");

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
