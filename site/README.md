# Learn2Rust site

The Learn2Rust landing page — a **SvelteKit** app (Svelte 5) prerendered to fully
static HTML/CSS/JS via `@sveltejs/adapter-static`. No external fonts, scripts, or
CDNs, so it works offline and inside a strict Content-Security-Policy.

## Local development

This project uses **pnpm** (pinned via the `packageManager` field in
`package.json`).

```powershell
cd site
pnpm install
pnpm dev           # dev server at http://localhost:5173
pnpm build         # prerender to site/build/
pnpm preview       # serve the production build
```

## Live status section

The "Where the course is now" section is generated from the real repository, not
hand-maintained. The `focus_forge_status` crate inspects the workspace (via `git`
and `cargo`) and writes `site/static/status.json`; the site fetches it at load.

`status.json` is a committed snapshot, so it updates only when regenerated. Refresh
it (requires the Rust toolchain) before committing site changes if progress has
moved on:

```powershell
cd site
pnpm status:refresh   # runs: cargo run -p focus_forge_status
```

The Vercel build only runs `pnpm build` (no Rust toolchain), so it serves whatever
`status.json` is committed — regeneration is a deliberate local step, not part of
the deploy.

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
├─ static/.nojekyll            # (only relevant to the GitHub Pages path)
├─ vercel.json                 # Vercel: static build, output = build/
├─ svelte.config.js            # adapter-static (served from domain root)
└─ vite.config.js
```

## Deploying to Vercel (primary)

The site deploys to Vercel via Git integration — no CLI needed, no GitHub Actions
minutes used.

1. At [vercel.com/new](https://vercel.com/new), import the `Learn2Rust` repo.
2. Set **Root Directory** to `site` (the SvelteKit app lives in a subfolder).
3. Vercel reads [`vercel.json`](vercel.json): build command `pnpm run build`,
   output directory `build`. Leave the rest at defaults and deploy.
4. Every push to `main` that touches `site/` then triggers an automatic
   redeploy; pull requests get preview deployments.

Because the site is served from the domain root on Vercel, `svelte.config.js` sets
no `paths.base`, and the static adapter emits relative asset paths.

## Deploying to GitHub Pages (alternative)

A GitHub Actions workflow ([`.github/workflows/pages.yml`](../.github/workflows/pages.yml))
can publish the same build to `https://soulwax.github.io/Learn2Rust/`. Note that
GitHub Pages serves the site under the `/Learn2Rust/` subpath, so if you use this
path you must set `kit.paths.base` to `/Learn2Rust` in `svelte.config.js`. The
`static/.nojekyll` file stops Pages from stripping the `_app/` directory.

## Editing

Content lives in `src/routes/+page.svelte`; styling in `src/app.css`. Keep the page
self-contained — do not add external asset URLs, or a strict host CSP may block them.
