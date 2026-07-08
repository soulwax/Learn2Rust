# Learn2Rust site

The Learn2Rust landing page — a single, dependency-free `index.html` (inline CSS
and JS, no external fonts, scripts, or CDNs, so it works offline and inside any
Content-Security-Policy).

## Enabling GitHub Pages (one time)

1. Push this folder and `.github/workflows/pages.yml` to `main` (already done if
   you cloned the repo after this was added).
2. In the repository on GitHub: **Settings → Pages → Build and deployment →
   Source → "GitHub Actions"**.
3. The [Deploy site to GitHub Pages](../.github/workflows/pages.yml) workflow runs
   on every push to `main` that touches `site/`, and on demand from the
   **Actions** tab.

The site will be published at:

```
https://soulwax.github.io/Learn2Rust/
```

## Editing

Open `index.html` and edit directly — there is no build step. To preview locally,
open the file in a browser, or serve the folder:

```powershell
# from the repo root
python -m http.server -d site 8000   # then open http://localhost:8000
```

The page is theme-aware (light/dark, following the OS by default) with a manual
toggle in the header. Keep it self-contained: do not add external asset URLs, or
the strict Pages/CSP environment may block them.
