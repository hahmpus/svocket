import { sveltekit } from '@sveltejs/kit/vite';

/** @type {import('vite').UserConfig} */
const config = {
  plugins: [sveltekit()],
//   ssr: {
//     noExternal: [/^@material\//],
//   },
//   resolve: {
//     dedupe: ['svelte'],
//   },
  css: {
    postcss: {}
  },
};

export default config;
