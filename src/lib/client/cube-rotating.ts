import { initWebGL, bindBufferToLocation } from "$lib/client/webgl-common"
import { mat4 } from "gl-matrix";
import { cube } from "./cube-def";

const vs = `#version 300 es
in vec4 aVertexPosition;
in vec4 aVertexColor;

uniform mat4 uModelViewMatrix;
uniform mat4 uProjectionMatrix;

out vec4 vColor;
  
void main() {
  gl_Position = uProjectionMatrix * uModelViewMatrix * aVertexPosition;
  vColor = aVertexColor;
}
`

const fs = `#version 300 es
precision mediump float;

in vec4 vColor;
out vec4 fragColor;

void main() {
  fragColor = vColor;
}
`

export class RotatingCube {
  private ctx;
  private locations;
  private buffers;
  private canvas;
  private delta = 0.0;
  private cube = cube();
  
  constructor(id: string) {
    const webgl = initWebGL(id, vs, fs, (msg) => console.log(msg));

    if (webgl === null) {
      throw new Error("failed to initialize WebGL")
    }

    const { ctx, canvas, program } = webgl;

    const locations = {
      attribute: {
        vp: ctx.getAttribLocation(program, "aVertexPosition"),
        vc: ctx.getAttribLocation(program, "aVertexColor"),
      },
      uniform: {
        mvm: ctx.getUniformLocation(program, "uModelViewMatrix"),
        pjm: ctx.getUniformLocation(program, "uProjectionMatrix"),
      }
    }

    const { positions, colors, indices } = this.cube;
    
    const buffers = {
      position: initBuffer(ctx, ctx.ARRAY_BUFFER, new Float32Array(positions)),
      colors: initBuffer(ctx, ctx.ARRAY_BUFFER, new Float32Array(colors)),
      indices: initBuffer(ctx, ctx.ELEMENT_ARRAY_BUFFER, new Uint16Array(indices)),
    }
    
    this.ctx = ctx;
    this.canvas = canvas;
    this.locations = locations;
    this.buffers = buffers;
  }

  tick(delta: number) {
    // if (this.locations.uniform.time !== null) {
    //   this.ctx.uniform1f(this.locations.uniform.time, timestamp);
    // }
    this.delta = delta;
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

    // if (this.locations.uniform.resolution) {
    //   this.ctx.uniform2fv(this.locations.uniform.resolution, [displayWidth, displayHeight]);
    // }

    this.ctx.viewport(0, 0, displayWidth, displayHeight);
  }
  
  draw() {
    this.resize();

    const ctx = this.ctx;

    ctx.clearColor(0.0, 0.0, 0.0, 1.0);
    ctx.clearDepth(1.0);
    ctx.enable(ctx.DEPTH_TEST);
    ctx.depthFunc(ctx.LEQUAL);

    ctx.clear(ctx.COLOR_BUFFER_BIT | ctx.DEPTH_BUFFER_BIT);

    bindBufferToLocation(ctx, 3, this.locations.attribute.vp, this.buffers.position);
    bindBufferToLocation(ctx, 4, this.locations.attribute.vc, this.buffers.colors);
    ctx.bindBuffer(ctx.ELEMENT_ARRAY_BUFFER, this.buffers.indices);

    const aspect = this.canvas.clientWidth / this.canvas.clientHeight;
    const m = matrices(aspect, this.delta);

    ctx.uniformMatrix4fv(this.locations.uniform.pjm, false, m.projectionMatrix);
    ctx.uniformMatrix4fv(this.locations.uniform.mvm, false, m.modelViewMatrix);
  
    {
      const vertexCount = this.cube.indices.length;
      const dataType = ctx.UNSIGNED_SHORT;
      const offset = 0;
      // const instanceCount = 1;
      this.ctx.drawElements(ctx.TRIANGLES, vertexCount, dataType, offset);
    }
  }
}

const initBuffer = (ctx: WebGL2RenderingContext, target: number, data: BufferSource | null) => {  
  const buf = ctx.createBuffer();
  ctx.bindBuffer(target, buf);
  ctx.bufferData(target, data, ctx.STATIC_DRAW);
  
  return buf;
}

const matrices = (aspect: number, rot: number) => {
  const fov = (45 * Math.PI) / 180; // field of view in radians
  const zNear = 0.1;
  const zFar = 100.0;

  const projectionMatrix = mat4.create();
  mat4.perspective(projectionMatrix, fov, aspect, zNear, zFar);

  const modelViewMatrix = mat4.create();
  mat4.translate(modelViewMatrix, modelViewMatrix, [-0.0, 0.0, -6.0]);
  mat4.rotate(modelViewMatrix, modelViewMatrix, rot * 1.0, [0, 0, 1]);  // around Z-axis
  mat4.rotate(modelViewMatrix, modelViewMatrix, rot * 0.7, [0, 1, 0]);  // around Y-axis
  mat4.rotate(modelViewMatrix, modelViewMatrix, rot * 0.3, [1, 0, 0]);  // around X-axis

  return {
    projectionMatrix, modelViewMatrix,
  }
}