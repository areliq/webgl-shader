<script lang="ts">
  import Slider from '$lib/components/Slider.svelte';

  //   const num = 3
  //   let current = 0;
  //   const prev = (idx: number) => () => { if (current > 0) {current--} console.log(current) };
  //   const next = (idx: number) => () => { if (current < num - 1) {current++} console.log(current) };
  //   const select = (idx: number) => () => { current = idx; console.log(current) };

  //   const slider = {
  //     num, current, prev, next, select
  //   }
  import vertexShader from '$lib/shaders/vertex_common.glsl';
  import fsTest from '$lib/shaders/fragment_test_uniform.glsl';
  import fsPeriodic from '$lib/shaders/periodic_perlin_noise.glsl';
  import fsPerlin from '$lib/shaders/perlin_noise.glsl';
  import fsVgNoise from '$lib/shaders/reference.glsl';
  import fsBrending from '$lib/shaders/blending.glsl';
  import fsFBM from '$lib/shaders/frac_brownian_motion.glsl';
  import fsDomainWarping from '$lib/shaders/domain_warping.glsl';
  import fsDomainWarpingRot from '$lib/shaders/domain_warping_rot.glsl';
  import fsConversion from '$lib/shaders/conversion.glsl';
  import fsBool from '$lib/shaders/bool.glsl'

  import { beforeUpdate } from 'svelte';
  import init, { GlBox } from '$lib/wasm/pkg';

  const shaders = [
    fsTest, fsPeriodic, fsPerlin, fsVgNoise, fsBrending, 
    fsFBM, fsDomainWarping, fsDomainWarpingRot, fsConversion,
    fsBool,
  ];

  const vShader = vertexShader;
  let current = 0;
  let fShader = shaders[current];
  let dynamic = true;

  beforeUpdate(async () => {
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

<Slider num={shaders.length} on:slide={(event) => (fShader = shaders[event.detail.next])}>
  <canvas id="canvas" />
</Slider>

<style>
  canvas {
    width: 420px;
    height: 340px;
    display: block;
    background-color: aquamarine;
  }
</style>
