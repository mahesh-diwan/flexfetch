import { defineConfig } from 'astro/config';

export default defineConfig({
  site: 'https://mahesh-diwan.github.io',
  base: '/flexfetch',
  output: 'static',
  build: {
    assets: 'assets',
  },
  compressHTML: true,
  prefetch: true,
  markdown: {
    shikiConfig: {
      theme: 'css-variables',
    },
  },
});