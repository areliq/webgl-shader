import { initWebGL, bindBufferToLocation } from '$lib/client/webgl-common';

const vertexShaderSource = `#version 300 es
// The individual position vertex
in vec2 position;
  
void main() {
    // the gl_Position is the final position in clip space 
    // after the vertex shader modifies it
    gl_Position = vec4(position, 0.0, 1.0);
}
`;

const fragmentShaderSource = `#version 300 es
precision highp float;

out vec4 fragColor;

uniform float u_time;
uniform vec2 u_resolution;


void main() {
    vec2 pos = gl_FragCoord.xy / u_resolution.xy;

    fragColor = vec4(abs(sin(u_time)), pos, 1.0);
}
`;

export class Whiteboard {
  private ctx;
  private locations;
  private buffers;
  private canvas;

  constructor(id: string) {
    const webgl = initWebGL(id, vertexShaderSource, fragmentShaderSource, (msg) =>
      console.log(msg)
    );

    if (webgl === null) {
      throw new Error('failed to initialize WebGL');
    }

    const { ctx, canvas, program } = webgl;

    const locations = {
      attribute: {
        vertexPosition: ctx.getAttribLocation(program, 'position')
      },
      uniform: {
        time: ctx.getUniformLocation(program, 'u_time'),
        resolution: ctx.getUniformLocation(program, 'u_resolution')
      }
    };

    const buffers = {
      position: initPositionBuffer(ctx)
    };

    this.ctx = ctx;
    this.canvas = canvas;
    this.locations = locations;
    this.buffers = buffers;
  }

  tick(timestamp: number) {
    if (this.locations.uniform.time !== null) {
      this.ctx.uniform1f(this.locations.uniform.time, timestamp);
    }
  }

  resize() {
    const displayWidth = this.canvas.clientWidth;
    const displayHeight = this.canvas.clientHeight;
    const canvasWidth = this.canvas.width;
    const canvasHeight = this.canvas.height;

    if (canvasWidth !== displayWidth || canvasHeight !== displayHeight) {
      this.canvas.width = displayWidth;
      this.canvas.height = displayHeight;
    }

    if (this.locations.uniform.resolution) {
      this.ctx.uniform2fv(this.locations.uniform.resolution, [displayWidth, displayHeight]);
    }

    this.ctx.viewport(0, 0, displayWidth, displayHeight);
  }

  draw() {
    this.resize();

    bindBufferToLocation(
      this.ctx,
      2,
      this.locations.attribute.vertexPosition,
      this.buffers.position
    );

    {
      const vertexCount = 6;
      const type = this.ctx.TRIANGLES;
      const offset = 0;
      this.ctx.drawArrays(type, offset, vertexCount);
    }
  }
}

const initPositionBuffer = (ctx: WebGL2RenderingContext) => {
  const triangle1 = [
    -1.0,
    -1.0, // left-bottom
    1.0,
    -1.0, // right-bottom
    -1.0,
    1.0 // left-top
  ];
  const triangle2 = [
    -1.0,
    1.0, // left-top
    1.0,
    -1.0, // right-bottom
    1.0,
    1.0 // right-top
  ];

  const positions = [...triangle1, ...triangle2];

  const positionBuffer = ctx.createBuffer();
  ctx.bindBuffer(ctx.ARRAY_BUFFER, positionBuffer);
  ctx.bufferData(ctx.ARRAY_BUFFER, new Float32Array(positions), ctx.STATIC_DRAW);

  return positionBuffer;
};
