import { initWebGL, bindBufferToLocation } from "$lib/client/webgl-common"
import { mat4 } from "gl-matrix";
import { cube } from "./cube-def";

const vsTexture = `#version 300 es
in vec4 a_position;
in vec2 a_texcoord;
in vec4 a_color;

uniform mat4 u_projection_matrix;
uniform mat4 u_model_view_matrix;

// a varying to pass the texture coordinates to the fragment shader
out vec2 v_texcoord;
// added
out vec4 v_color;

void main() {
  // Multiply the position by the matrix.
  gl_Position = u_projection_matrix * u_model_view_matrix * a_position;

  // Pass the texcoord to the fragment shader.
  v_texcoord = a_texcoord;

  // pass attribute color
  v_color = a_color;
}
`

const fsTexture = `#version 300 es
precision highp float;

// Passed in from the vertex shader.
in vec2 v_texcoord;
// added
in vec4 v_color;

// The texture.
uniform sampler2D u_texture;

out vec4 fragColor;

void main() {
  fragColor = texture(u_texture, v_texcoord) * v_color;
}
`

const vsColor = `#version 300 es
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

const fsColor = `#version 300 es
precision highp float;

in vec4 vColor;
out vec4 fragColor;

void main() {
  fragColor = vColor;
}
`

// const assert = <T>(obj: T | null, name = "obj"): obj is T => {
//     if (obj === null) {
//         throw new Error(`${name} is null`)
//     } else {
//         return true
//     }
// }

export class TextureCube {
  private ctx;
  private locations;
  private buffers;
  private canvas;
  // private texture;
  private delta = 0.0;
  private cube = cube();
  
  constructor(id: string) {
    const webgl = initWebGL(id, vsTexture, fsTexture, (msg) => console.log(msg));

    if (webgl === null) {
      throw new Error("failed to initialize WebGL")
    }

    const { ctx, canvas, program } = webgl;

    // Create a vertex array object (attribute state)
    // const vao = ctx.createVertexArray();
    // and make it the one we're currently working with
    // ctx.bindVertexArray(vao);

    const locations = {
      attribute: {
        vpos: ctx.getAttribLocation(program, "a_position"),
        vtex: ctx.getAttribLocation(program, "a_texcoord"),
        vcol: ctx.getAttribLocation(program, "a_color"),
        // vp: ctx.getAttribLocation(program, "aVertexPosition"),
        // vc: ctx.getAttribLocation(program, "aVertexColor"),
      },
      uniform: {
        pjm: ctx.getUniformLocation(program, "u_projection_matrix"),
        mvm: ctx.getUniformLocation(program, "u_model_view_matrix"),
        sampler: ctx.getUniformLocation(program, "u_texture"),
        // mvm: ctx.getUniformLocation(program, "uModelViewMatrix"),
        // pjm: ctx.getUniformLocation(program, "uProjectionMatrix"),
      }
    }

    const { positions, colors, indices, texcoord } = this.cube;

    // console.log(this.cube)
    
    const buffers = {
      position: initBuffer(ctx, ctx.ARRAY_BUFFER, new Float32Array(positions)),
      colors: initBuffer(ctx, ctx.ARRAY_BUFFER, new Float32Array(colors)),
      texture: initBuffer(ctx, ctx.ARRAY_BUFFER, new Float32Array(texcoord)),
      indices: initBuffer(ctx, ctx.ELEMENT_ARRAY_BUFFER, new Uint16Array(indices)),
    }

    const texture = initTexture(ctx);

    ctx.activeTexture(ctx.TEXTURE0);
    ctx.bindTexture(ctx.TEXTURE_2D, texture);

    // Tell the shader to get the texture from texture unit 0
    ctx.uniform1i(locations.uniform.sampler, 0);

    // Set the parameters so we don't need mips and so we're not filtering
    // and we don't repeat at the edges
    ctx.texParameteri(ctx.TEXTURE_2D, ctx.TEXTURE_WRAP_S, ctx.CLAMP_TO_EDGE);
    ctx.texParameteri(ctx.TEXTURE_2D, ctx.TEXTURE_WRAP_T, ctx.CLAMP_TO_EDGE);
    ctx.texParameteri(ctx.TEXTURE_2D, ctx.TEXTURE_MIN_FILTER, ctx.NEAREST);
    ctx.texParameteri(ctx.TEXTURE_2D, ctx.TEXTURE_MAG_FILTER, ctx.NEAREST);

    // Flip image pixels into the bottom-to-top order that WebGL expects.
    ctx.pixelStorei(ctx.UNPACK_FLIP_Y_WEBGL, true);
    
    this.ctx = ctx;
    this.canvas = canvas;
    this.locations = locations;
    this.buffers = buffers;
    // this.texture = texture;
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

  loadImageToTexture(image: HTMLImageElement) {
    const ctx = this.ctx;
    const level = 0;
    const internalFormat = ctx.RGBA;
    const srcFormat = ctx.RGBA;
    const srcType = ctx.UNSIGNED_BYTE;
    // ctx.bindTexture(ctx.TEXTURE_2D, this.texture);
    ctx.texImage2D(ctx.TEXTURE_2D, level, internalFormat, srcFormat, srcType, image);
    console.log("image load to texture")
  }
  
  draw() {
    this.resize();

    const ctx = this.ctx;

    ctx.clearColor(0.0, 0.0, 0.0, 1.0);
    ctx.clearDepth(1.0);
    ctx.enable(ctx.DEPTH_TEST);
    ctx.depthFunc(ctx.LEQUAL);

    ctx.clear(ctx.COLOR_BUFFER_BIT | ctx.DEPTH_BUFFER_BIT);

    bindBufferToLocation(ctx, 3, this.locations.attribute.vpos, this.buffers.position);
    bindBufferToLocation(ctx, 2, this.locations.attribute.vtex, this.buffers.texture);
    bindBufferToLocation(ctx, 4, this.locations.attribute.vcol, this.buffers.colors);
    // bindBufferToLocation(ctx, 3, this.locations.attribute.vp, this.buffers.position);
    // bindBufferToLocation(ctx, 4, this.locations.attribute.vc, this.buffers.colors);
    ctx.bindBuffer(ctx.ELEMENT_ARRAY_BUFFER, this.buffers.indices);

    // ctx.activeTexture(ctx.TEXTURE0);
    // ctx.bindTexture(ctx.TEXTURE_2D, this.texture);
  
    // // bind the texture to unit 0
    // ctx.uniform1i(this.locations.uniform.sampler, 0); 

    const aspect = this.canvas.clientWidth / this.canvas.clientHeight;
    const m = matrices(aspect, this.delta);

    ctx.uniformMatrix4fv(this.locations.uniform.pjm, false, m.projectionMatrix);
    ctx.uniformMatrix4fv(this.locations.uniform.mvm, false, m.modelViewMatrix);
  
    {
      const vertexCount = this.cube.indices.length;
      const dataType = ctx.UNSIGNED_SHORT;
      const offset = 0;
      // const instanceCount = 1;
      this.ctx.drawElements(ctx.TRIANGLES, vertexCount, dataType, offset);  // ctx.LINES
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

const initTexture = (ctx: WebGL2RenderingContext) => {
  const texture = ctx.createTexture();
  ctx.bindTexture(ctx.TEXTURE_2D, texture);

  const level = 0;
  const internalFormat = ctx.RGBA;
  const width = 1;
  const height = 1;
  const border = 0;
  const srcFormat = ctx.RGBA;
  const srcType = ctx.UNSIGNED_BYTE;
  const pixel = new Uint8Array([0, 0, 255, 255]);  // blue: temporary color

  ctx.texImage2D(
    ctx.TEXTURE_2D, level, internalFormat, width, 
    height, border, srcFormat, srcType, pixel
  );

  return texture;
}
