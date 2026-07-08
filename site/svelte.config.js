import adapter from '@sveltejs/adapter-static';

// GitHub Pages serves this project site under /Learn2Rust/, so we set a base
// path in production. Locally (dev/preview) the base is empty.
const dev = process.argv.includes('dev');

/** @type {import('@sveltejs/kit').Config} */
const config = {
  kit: {
    // adapter-static prerenders the whole site to plain HTML/CSS/JS — no server.
    adapter: adapter({
      pages: 'build',
      assets: 'build',
      fallback: undefined,
      precompress: false,
      strict: true
    }),
    paths: {
      base: dev ? '' : '/Learn2Rust'
    }
  }
};

export default config;
