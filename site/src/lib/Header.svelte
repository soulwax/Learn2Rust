<script>
  import { cycleTheme, themeIcon, theme } from './theme.svelte.js';
</script>

<header class="site">
  <div class="wrap">
    <span class="brand"><span class="logo">🦀</span> Learn2Rust</span>
    <nav>
      <a href="#start">Get started</a>
      <a href="#layout">Layout</a>
      <a href="#commands">Commands</a>
      <a href="#status">Status</a>
    </nav>
    <button
      class="theme-toggle"
      onclick={cycleTheme}
      title={`Theme: ${theme.value} (click to change)`}
      aria-label="Toggle color theme">{themeIcon()}</button>
  </div>
</header>
