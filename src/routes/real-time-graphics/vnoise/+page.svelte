<script lang="ts">
  import fragmentShader from '$lib/shaders/3_0_value_noise.glsl';
  import GlCanvas from '$lib/components/GlCanvas.svelte'
</script>
  
<svelte:head>
  <title>Value Noise</title>
  <meta name="description" content="WebGL Shader Value NOise" />
</svelte:head>

<GlCanvas fShader={fragmentShader} />
