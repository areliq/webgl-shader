<script lang="ts">
  import fragmentShader from '$lib/shaders/4_1_value_range.glsl';
  import GlCanvas from '$lib/components/GlCanvas.svelte'
</script>
  
<svelte:head>
  <title>Noise Range</title>
  <meta name="description" content="WebGL Shader Grad/Value Noise Range" />
</svelte:head>

<GlCanvas fShader={fragmentShader} />
