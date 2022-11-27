<script>
    import { onMount } from 'svelte';
    import init, { render as renderWebGL } from '$lib/wasm/pkg';
  
    const vertexShader = `
    attribute vec4 aVertexPosition;
    attribute vec4 aVertexColor;
    uniform mat4 uModelViewMatrix;
    uniform mat4 uProjectionMatrix;
    varying lowp vec4 vColor;
  
    void main() {
      gl_Position = uProjectionMatrix * uModelViewMatrix * aVertexPosition;
      vColor = aVertexColor;
    }`
  
    const fragmentShader = `
    varying lowp vec4 vColor;
    void main() {
      gl_FragColor = vColor;
    }`
  
  
    onMount(async () => {
      await init();
    
      renderWebGL(vertexShader, fragmentShader);
    })
</script>
  
<svelte:head>
  <title>3 - Using shaders to apply color in WebGL</title>
  <meta name="description" content="WebGL Shader App" />
</svelte:head>
  
  <canvas id="canvas" width="640px" height="320px"></canvas>
  