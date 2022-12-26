<script lang="ts">
  import vertexShader from '$lib/shaders/vertex_common.glsl';
  import fragmentShader from '$lib/shaders/fragment_test_uniform.glsl';
  import { onMount } from 'svelte';
  import init, { GlBox } from '$lib/wasm/pkg';

  export let vShader = vertexShader;
  export let fShader = fragmentShader;
  export let dynamic = true;

  onMount(async () => {
    await init();

    const square = GlBox.new('canvas', dynamic, vShader, fShader);

    const renderLoop: FrameRequestCallback = (timestamp) => {
      square.tick(timestamp / 1000);
      square.draw();

      requestAnimationFrame(renderLoop);
    };

    requestAnimationFrame(renderLoop);
  });
</script>

<canvas id="canvas" />

<style>
  #canvas {
    width: 100vw;
    height: 100vh;
    display: block;
  }
</style>
