export const initWebGL = (
    id: string, 
    vertexShaderSource: string, 
    fragmentShaderSource: string, 
    onError: (msg: string) => void = () => {}
) => {
    const canvas = document.querySelector<HTMLCanvasElement>(id);
    
    if (canvas === null) {
      onError("canvas element does not found.")
      return null;
    }
    
    const gl2 = canvas.getContext("webgl2");
    
    if (gl2 === null) {
      onError("failed to get WebGL2 context.");
      return null;
    }
  
    const vertexShader = loadShader(gl2, gl2.VERTEX_SHADER, vertexShaderSource, onError);
    const fragmentShader = loadShader(gl2, gl2.FRAGMENT_SHADER, fragmentShaderSource, onError);
    
    if (vertexShader === null) {
      onError("failed to load vertex shader.");
      return null;
    }
    
    if (fragmentShader === null) {
      onError("failed to load fragment shader.");
      return null;
    }
    
    const program = gl2.createProgram();
    
    if (program === null) {
      onError("failed to create shader program.");
      return null;
    }
    
    gl2.attachShader(program, vertexShader);
    gl2.attachShader(program, fragmentShader);
    gl2.linkProgram(program);
    
    if (!gl2.getProgramParameter(program, gl2.LINK_STATUS)) {
      const msg = `failed to link shader program: ${gl2.getProgramInfoLog(program)}`;
      onError(msg);
      return null;
    }
  
    gl2.useProgram(program);
  
    return {
      ctx: gl2, canvas, program,
    }
  }
  
const loadShader = (
    ctx: WebGL2RenderingContext, 
    shaderType: number, 
    source: string, 
    onError: (msg: string) => void = () => {}
) => {
    const shader = ctx.createShader(shaderType);
    
    if (shader === null) {
      const msg = `failed to create the shader`;
      onError(msg);
      return null;
    }
    
    ctx.shaderSource(shader, source);
    ctx.compileShader(shader);
    
    if (!ctx.getShaderParameter(shader, ctx.COMPILE_STATUS)) {
      const msg = `an error occured while compiling the shader: ${ctx.getShaderInfoLog(shader)}`;
      onError(msg);
      ctx.deleteShader(shader);
      return null;
    }
    
    return shader
}

export const initBuffer = (ctx: WebGL2RenderingContext, target: number, data: BufferSource | null) => {  
  const buf = ctx.createBuffer();
  ctx.bindBuffer(target, buf);
  ctx.bufferData(target, data, ctx.STATIC_DRAW);
  
  return buf;
}

export const bindBufferToLocation = (ctx: WebGL2RenderingContext, numComponents: number, location: number, buffer: WebGLBuffer | null) => {
  const type = ctx.FLOAT;
  const normalize = false;
  const stride = 0;
  const offset = 0;
  
  ctx.bindBuffer(ctx.ARRAY_BUFFER, buffer);
  ctx.vertexAttribPointer(
    location,
    numComponents,
    type,
    normalize,
    stride,
    offset
  );
  
  ctx.enableVertexAttribArray(location);
}
