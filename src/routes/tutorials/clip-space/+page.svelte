<script>
    import { onMount } from 'svelte';
    import init, { render_boxes_to_clipspace_directly } from '$lib/wasm/pkg';
  
    onMount(async () => {
      await init();
      render_boxes_to_clipspace_directly("canvas");
    })
</script>
  
<svelte:head>
  <title>clip space</title>
  <meta name="description" content="WebGL Shader App" />
</svelte:head>

<canvas id="canvas" width="540px" height="340px"></canvas>
