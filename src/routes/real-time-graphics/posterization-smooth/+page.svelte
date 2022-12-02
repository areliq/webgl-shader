<script>
    import { onMount } from 'svelte';
    import init, { GlBox } from '$lib/wasm/pkg';
  
    const vertex_shader = "#version 300 es" + `
    // The individual position vertex
    in vec2 position;
  
    void main() {
      // the gl_Position is the final position in clip space 
      // after the vertex shader modifies it
      gl_Position = vec4(position, 0.0, 1.0);
    }`;
  
    const fragment_shader = "#version 300 es" + `
    precision mediump float;
    out vec4 fragColor;
    uniform vec2 u_resolution;
    uniform float u_time;
    int channel;

    void main() {
      vec2 pos = gl_FragCoord.xy / u_resolution.xy;

      vec3[4] col4 = vec3[](
        vec3(1.0, 0.0, 0.0),
        vec3(1.0, 1.0, 0.0),
        vec3(1.0, 0.0, 1.0),
        vec3(1.0, 1.0, 1.0)
      );
      float n = 4.0;
      pos *= n;
      channel = int(2.0 * gl_FragCoord.x / u_resolution.x);

      if (channel == 0){   //left: step
        pos = floor(pos) + step(0.5, fract(pos));
      } else {    //right: smoothstep
        float thr = 0.25 * sin(u_time);
        pos = floor(pos) + smoothstep(0.25 + thr, 0.75 - thr, fract(pos));
      }

      pos /= n;

      vec3 col = mix(mix(col4[0], col4[1], pos.x), mix(col4[2], col4[3], pos.x), pos.y);

      fragColor = vec4(col, 1.0);

      // fragColor = vec4(abs(sin(u_time)), 0.0, 0.0, 1.0);
    }`;
    
    onMount(async () => {
      await init();
      
      const square = GlBox.new("canvas", false, vertex_shader, fragment_shader);

      const renderLoop = () => {
        let current = Date.now();
        square.tick(current);
        square.draw();

        requestAnimationFrame(renderLoop);
      };

      requestAnimationFrame(renderLoop);

    })
  </script>
  
  <svelte:head>
    <title>Hello World</title>
    <meta name="description" content="WebGL Shader App" />
  </svelte:head>
  
  <style>
    canvas {
      width: 100vw;
      height: 100vh;
      display: block;
    }
  </style>
  
  <canvas id="canvas"></canvas>