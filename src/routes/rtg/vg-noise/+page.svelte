<script lang="ts">
  import fragmentShader from '$lib/shaders/reference.glsl';
  import GlCanvas from '$lib/components/GlCanvas.svelte'
</script>
  
<svelte:head>
  <title>WebGL Canvas</title>
  <meta name="description" content="WebGL Shader App" />
</svelte:head>

<GlCanvas fShader={fragmentShader} />
