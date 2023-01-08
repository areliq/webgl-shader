import { initWebGL, bindBufferToLocation, initBuffer } from '$lib/client/webgl-common';
import { mat4 } from 'gl-matrix';
import { cube } from './cube-def';

const vs = `#version 300 es
in vec2 a_position;
in vec2 a_texcoord;

uniform vec2 u_resolution;

// a varying to pass the texture coordinates to the fragment shader
out vec2 v_texcoord;

void main() {
  // convert the position from pixels to [0.0, 1.0]
  vec2 zero_one_space = a_position / u_resolution;
  // convert the position from [0.0, 1.0] to [0.0, 2.0]
  vec2 zero_two_space = zero_one_space * 2.0;
  // convert to clipspace: [-1.0, 1.0]
  vec2 clipspace = zero_two_space - 1.0;

  gl_Position = vec4(clipspace * vec2(1, -1), 0.0, 1.0);

  // Pass the texcoord to the fragment shader.
  v_texcoord = a_texcoord;
}
`;

const fs = `#version 300 es
precision highp float;
// our texture
uniform sampler2D u_image;
// the texCoords passed in from the vertex shader.
in vec2 v_texcoord;

// we need to declare an output for the fragment shader
out vec4 fragColor;

void main() {
  // Look up a color from the texture.
  fragColor = texture(u_image, v_texcoord);
}
`;

const texCoord = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0];

export class TextureBoard {
  private ctx;
  private locations;
  private buffers;
  private canvas;
  //   private texture;
  //   private image;

  constructor(id: string) {
    const webgl = initWebGL(id, vs, fs, (msg) => console.log(msg));

    if (webgl === null) {
      throw new Error('failed to initialize WebGL');
    }

    const { ctx, canvas, program } = webgl;

    const locations = {
      attribute: {
        vpos: ctx.getAttribLocation(program, 'a_position'),
        vtex: ctx.getAttribLocation(program, 'a_texcoord')
      },
      uniform: {
        resolution: ctx.getUniformLocation(program, 'u_resolution'),
        sampler: ctx.getUniformLocation(program, 'u_image')
      }
    };

    const buffers = {
      position: initBuffer(
        ctx,
        ctx.ARRAY_BUFFER,
        new Float32Array(rect(0, 0, canvas.width, canvas.height))
      ),
      // colors: initBuffer(ctx, ctx.ARRAY_BUFFER, new Float32Array(colors)),
      // indices: initBuffer(ctx, ctx.ELEMENT_ARRAY_BUFFER, new Uint16Array(indices)),
      texture: initBuffer(ctx, ctx.ARRAY_BUFFER, new Float32Array(texCoord))
    };

    // init with temporary texture
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
    // ctx.pixelStorei(ctx.UNPACK_FLIP_Y_WEBGL, true);

    // todo: lazy loading
    // const level = 0;
    // const internalFormat = ctx.RGBA;
    // const srcFormat = ctx.RGBA;
    // const srcType = ctx.UNSIGNED_BYTE;
    // ctx.texImage2D(ctx.TEXTURE_2D, level, internalFormat, srcFormat, srcType, image);

    this.ctx = ctx;
    this.canvas = canvas;
    this.locations = locations;
    this.buffers = buffers;
    // this.texture = texture;
    // this.image = image;
  }

  loadImage(image: HTMLImageElement) {
    const ctx = this.ctx;

    const position = new Float32Array(rect(0, 0, image.width, image.height));
    this.buffers.position = initBuffer(ctx, ctx.ARRAY_BUFFER, position);

    const level = 0;
    const internalFormat = ctx.RGBA;
    const srcFormat = ctx.RGBA;
    const srcType = ctx.UNSIGNED_BYTE;
    ctx.texImage2D(ctx.TEXTURE_2D, level, internalFormat, srcFormat, srcType, image);
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

    const ctx = this.ctx;

    ctx.clearColor(0.0, 0.0, 0.0, 1.0);
    ctx.clearDepth(1.0);
    ctx.enable(ctx.DEPTH_TEST);
    ctx.depthFunc(ctx.LEQUAL);

    ctx.clear(ctx.COLOR_BUFFER_BIT | ctx.DEPTH_BUFFER_BIT);

    bindBufferToLocation(ctx, 2, this.locations.attribute.vpos, this.buffers.position);
    bindBufferToLocation(ctx, 2, this.locations.attribute.vtex, this.buffers.texture);

    {
      const vertexCount = 6;
      const type = ctx.TRIANGLES;
      const offset = 0;
      ctx.drawArrays(type, offset, vertexCount);
    }
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
  const pixel = new Uint8Array([0, 0, 255, 255]); // temporary color

  ctx.texImage2D(
    ctx.TEXTURE_2D,
    level,
    internalFormat,
    width,
    height,
    border,
    srcFormat,
    srcType,
    pixel
  );

  return texture;
};

const rect = (x: number, y: number, w: number, h: number) => {
  const x0 = x;
  const x1 = x + w;
  const y0 = y;
  const y1 = y + h;

  return [x0, y0, x1, y0, x0, y1, x0, y1, x1, y0, x1, y1];
};
