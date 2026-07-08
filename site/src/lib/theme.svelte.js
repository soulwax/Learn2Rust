// Reactive theme state, ported from the original page's theme toggle:
// cycles auto -> light -> dark, persists to localStorage under "l2r-theme",
// and stamps data-theme on <html> so the CSS overrides win over the media query.
import { browser } from '$app/environment';

const KEY = 'l2r-theme';
const ORDER = ['auto', 'light', 'dark'];
const ICON = { auto: '◐', light: '☀', dark: '☾' }; // ◐ ☀ ☾

function initial() {
  if (!browser) return 'auto';
  try {
    return localStorage.getItem(KEY) || 'auto';
  } catch {
    return 'auto';
  }
}

// Svelte 5 runes: a module-level reactive value shared across components.
export const theme = $state({ value: initial() });

/** The glyph shown on the toggle button for the current theme. */
export function themeIcon() {
  return ICON[theme.value] || ICON.auto;
}

/** Advance to the next theme in the cycle and persist it. */
export function cycleTheme() {
  const next = ORDER[(ORDER.indexOf(theme.value) + 1) % ORDER.length];
  theme.value = next;
  if (browser) {
    document.documentElement.setAttribute('data-theme', next);
    try {
      localStorage.setItem(KEY, next);
    } catch {
      /* ignore storage failures (private mode, etc.) */
    }
  }
}
