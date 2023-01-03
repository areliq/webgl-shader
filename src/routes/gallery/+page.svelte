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
  import fsDist from '$lib/shaders/dist_first.glsl'
  import fsDistImproved from '$lib/shaders/dist_first_improved.glsl'

  import { beforeUpdate } from 'svelte';
  import init, { GlBox } from '$lib/wasm/pkg';

  // const shaders = [
  //   fsTest, fsPeriodic, fsPerlin, fsVgNoise, fsBrending, 
  //   fsFBM, fsDomainWarping, fsDomainWarpingRot, fsConversion,
  //   fsBool, fsDist, fsDistImproved,
  // ];

  const shaders = [
    { id: 0, source: fsTest, title: "Test", },
    { id: 1, source: fsVgNoise, title: "Value/Gradient Noise", },
    { id: 2, source: fsPerlin, title: "Perlin Noise", },
    { id: 3, source: fsPeriodic, title: "Perlin Noise (Periodic)", },
    { id: 4, source: fsFBM, title: "Fractional Brownian Motion", },
    { id: 5, source: fsDomainWarping, title: "Domain Warping", },
    { id: 6, source: fsDomainWarpingRot, title: "Domain Warping (+Rot)", },
    { id: 7, source: fsConversion, title: "Gradation Conversion", },
    { id: 8, source: fsBrending, title: "Image Brending", },
    { id: 9, source: fsBool, title: "Boolean Operation", },
    { id: 10, source: fsDist, title: "First Nearest Neighbor Distance", },
    { id: 11, source: fsDistImproved, title: "First Nearest Neighbor Distance (Calc Improved)", },
  ];

  const vShader = vertexShader;
  // let current = 0;
  let current = shaders[0]
  // let fShader = current.source;
  let dynamic = true;

  beforeUpdate(async () => {
    await init();

    const square = GlBox.new('canvas', dynamic, vShader, current.source);

    const renderLoop: FrameRequestCallback = (timestamp) => {
      square.tick(timestamp / 1000);
      square.draw();

      requestAnimationFrame(renderLoop);
    };

    requestAnimationFrame(renderLoop);
  });
</script>

<!-- 
  <Slider num={shaders.length} on:slide={(event) => (fShader = shaders[event.detail.next].source)}>
    <canvas id="canvas" />
  </Slider> 
-->

<div class="container">
  <div class="item">
    <canvas id="canvas" />
  </div>
  <div class="item">
    <fieldset class="menu">
      <legend class="legend"><b>Shaders</b></legend>
      {#each shaders as shader}
      {@const optionID = `shader-${shader.id}`}
      <div class="option">
        <input type="radio" id={optionID} bind:group={current} name="shaders" value={shader} />
        <label for={optionID}>{shader.title}</label>
      </div>
      {/each}
    </fieldset>
  </div>
</div>

<style>
  #canvas {
    width: 420px;
    height: 340px;
    display: block;
    background-color: aquamarine;
  }
  .container {
    display: flex;
  }
  .item {
    margin: 8px;
  }
  .menu {
    background-color: #F8F8F8;
  }
  .legend {
    background-color: #F8F8F8;
    border: 1.6px solid #666;
    padding: 4px 8px;
  }
</style>
