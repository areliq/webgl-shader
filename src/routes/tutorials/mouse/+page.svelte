<script lang="ts">
  import vertexShader from '$lib/shaders/vertex_common.glsl';
  import fragmentShader from '$lib/shaders/mouse.glsl';
  import { onMount } from 'svelte';
  import init, { MouseBox } from '$lib/wasm/pkg';

  export let vShader = vertexShader;
  export let fShader = fragmentShader;
  export let dynamic = true;

  let m = { x: 0.0, y: 0.0 };

  const handleMouseMoveOnCanvas = (ev: any /* TODO: typing */) => {
    const target = ev.target;
    const rect = target.getBoundingClientRect();

    const relativeMousePosition = {
      x: ev.clientX - rect.left,
      y: ev.clientY - rect.top
    };

    m.x = (relativeMousePosition.x * target.width) / target.clientWidth;
    m.y = (relativeMousePosition.y * target.height) / target.clientHeight;

    // console.log(`mouse updates: (${m.x}, ${m.y})`);
  };

  onMount(async () => {
    await init();

    const square = MouseBox.new('canvas', dynamic, vShader, fShader);

    const renderLoop: FrameRequestCallback = (timestamp) => {
      square.tick(timestamp / 1000, m.x, m.y);
      square.draw();

      requestAnimationFrame(renderLoop);
    };

    requestAnimationFrame(renderLoop);
  });
</script>

<canvas id="canvas" on:mousemove={handleMouseMoveOnCanvas} />

<style>
  #canvas {
    width: 100vw;
    height: 100vh;
    display: block;
  }
</style>
