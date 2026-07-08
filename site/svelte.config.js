import adapter from '@sveltejs/adapter-static';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  kit: {
    // The whole site is prerendered (see src/routes/+layout.js), so adapter-static
    // emits plain HTML/CSS/JS with no serverless functions. Vercel detects
    // SvelteKit and serves this static output directly.
    adapter: adapter({
      pages: 'build',
      assets: 'build',
      fallback: undefined,
      precompress: false,
      strict: true
    })
    // No `paths.base`: on Vercel the site is served from the domain root.
  }
};

export default config;
