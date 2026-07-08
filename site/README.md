# Learn2Rust site

The Learn2Rust landing page — a **SvelteKit** app (Svelte 5) prerendered to fully
static HTML/CSS/JS via `@sveltejs/adapter-static`, then deployed to GitHub Pages.
No external fonts, scripts, or CDNs, so it works offline and inside a strict
Content-Security-Policy.

## Local development

```powershell
cd site
npm install
npm run dev        # dev server at http://localhost:5173
npm run build      # prerender to site/build/
npm run preview    # serve the production build at http://localhost:4173/Learn2Rust/
```

## Structure

```
site/
├─ src/
│  ├─ app.html                 # HTML shell + pre-paint theme script
│  ├─ app.css                  # all styling (design tokens, layout, components)
│  ├─ lib/
│  │  ├─ theme.svelte.js       # reactive theme store (auto/light/dark)
│  │  ├─ Header.svelte         # sticky nav + theme toggle
│  │  └─ Footer.svelte
│  └─ routes/
│     ├─ +layout.js            # prerender = true
│     ├─ +layout.svelte        # imports app.css
│     └─ +page.svelte          # the landing page content
├─ static/.nojekyll            # stops GitHub Pages stripping the _app/ dir
├─ svelte.config.js            # adapter-static, base path /Learn2Rust
└─ vite.config.js
```

## Deploying to GitHub Pages (one time)

1. Push `site/` and `.github/workflows/pages.yml` to `main`.
2. In the repository on GitHub: **Settings → Pages → Build and deployment →
   Source → "GitHub Actions"**.
3. The [Deploy site to GitHub Pages](../.github/workflows/pages.yml) workflow then
   runs on every push to `main` that touches `site/` (and on demand from the
   **Actions** tab). It runs `npm ci && npm run build` and publishes `site/build`.

The site is published at:

```
https://soulwax.github.io/Learn2Rust/
```

## Base path

The site is served under `/Learn2Rust/`, so `svelte.config.js` sets
`kit.paths.base` to `/Learn2Rust` for production builds (empty in dev). The static
adapter emits relative asset paths, so the build works both at that subpath and if
previewed locally.

## Editing

Content lives in `src/routes/+page.svelte`; styling in `src/app.css`. Keep the page
self-contained — do not add external asset URLs, or the strict Pages/CSP
environment may block them.
