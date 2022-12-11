<script lang="ts">
  import fragmentShader from '$lib/shaders/4_0_grad_noise.glsl';
  import GlCanvas from '$lib/components/GlCanvas.svelte'
</script>
  
<svelte:head>
  <title>Grad Noise</title>
  <meta name="description" content="WebGL Shader Grad Noise" />
</svelte:head>

<GlCanvas fShader={fragmentShader} />
