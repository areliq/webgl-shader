import { sveltekit } from '@sveltejs/kit/vite';
import type { UserConfig } from 'vite';

import { createFilter } from '@rollup/pluginutils';

function glsl(options = { include: ['**/*.glsl'], exclude: [] as string[] }) {
  const filter = createFilter(options.include, options.exclude);

  return {
    name: 'glsl',
    transform(source: string, id: string) {
      if (!filter(id)) return;

      const code = `export default \`${source}\``;

      return { code };
    }
  };
}

const config: UserConfig = {
  plugins: [sveltekit(), glsl()]
};

export default config;
