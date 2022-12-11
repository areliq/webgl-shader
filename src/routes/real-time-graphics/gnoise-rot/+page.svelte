<script lang="ts">
  import fragmentShader from '$lib/shaders/4_2_rot_noise.glsl';
  import GlCanvas from '$lib/components/GlCanvas.svelte'
</script>
  
<svelte:head>
  <title>Rot Noise</title>
  <meta name="description" content="WebGL Shader Rot Noise" />
</svelte:head>

<GlCanvas fShader={fragmentShader} />
