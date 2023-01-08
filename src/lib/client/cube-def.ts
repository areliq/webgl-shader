const frontFace = [-1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0];

const backFace = [-1.0, -1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0];

const topFace = [-1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0];

const bottomFace = [-1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0];

const rightFace = [1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0];

const leftFace = [-1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0];

const triangles = (start: number) => [start, start + 1, start + 2, start, start + 2, start + 3];

const faceColors = [
  [0.8, 0.8, 0.8, 1.0], // Front face: white
  [0.8, 0.0, 0.0, 1.0], // Back face: red
  [0.0, 0.8, 0.0, 1.0], // Top face: green
  [0.0, 0.0, 0.8, 1.0], // Bottom face: blue
  [0.8, 0.8, 0.0, 1.0], // Right face: yellow
  [0.8, 0.0, 0.8, 1.0] // Left face: purple
];

// defines the texture coordinates corresponding to each vertex of each face.
// Note that the texture coordinates range from 0.0 to 1.0;
// the dimensions of textures are normalized to a range of 0.0 to 1.0 regardless of their actual size,
// for the purpose of texture mapping.
const textureCoordinate = [
  // 0.0, 0.0,
  // 0.0, 1.0,
  // 1.0, 0.0,
  // 0.0, 1.0,
  // 1.0, 1.0,
  // 1.0, 0.0
  // Front
  0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0
  // Back
  // 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0,
  // // Top
  // 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0,
  // // Bottom
  // 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0,
  // // Right
  // 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0,
  // // Left
  // 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0,
];

export const cube = () => {
  return {
    positions: [...frontFace, ...backFace, ...topFace, ...bottomFace, ...rightFace, ...leftFace],
    indices: [0, 4, 8, 12, 16, 20].flatMap((n) => triangles(n)),
    colors: faceColors.flatMap((c) => [c, c, c, c]).flat(),
    texcoord: [...Array<number[]>(6)].fill(textureCoordinate).flat()
  };
};
