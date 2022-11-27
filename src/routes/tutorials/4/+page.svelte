<script>
    import { onMount } from 'svelte';
    import init, { RotatingSquare } from '$lib/wasm/pkg';
  
    onMount(async () => {
      await init();
    
      const square = RotatingSquare.new("canvas");

      const renderLoop = () => {
        square.render();
        square.tick();

        requestAnimationFrame(renderLoop);
      };

      requestAnimationFrame(renderLoop);
    })
</script>
  
<svelte:head>
  <title>4 - requestAnimationFrame</title>
  <meta name="description" content="WebGL Shader App" />
</svelte:head>

<canvas id="canvas" width="640px" height="320px"></canvas>
