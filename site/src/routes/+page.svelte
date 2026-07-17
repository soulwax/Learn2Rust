<script>
  import Header from '$lib/Header.svelte';
  import Footer from '$lib/Footer.svelte';
  import StatusSection from '$lib/StatusSection.svelte';
</script>

<svelte:head>
  <title>Learn2Rust — learn Rust by building something useful</title>
  <meta
    name="description"
    content="A clone-and-learn Rust course for returning developers. Learn Rust by building Focus Forge, a real task and note workbench, one chapter at a time." />
</svelte:head>

<Header />

<main>
  <div class="wrap">
    <section class="hero" style="border-top:none">
      <span class="eyebrow">🦀 Clone-and-learn Rust course</span>
      <h1>Learn Rust by building<br /><span class="grad">something useful.</span></h1>
      <p class="lead">
        Learn2Rust is a course-as-repository for developers returning to code — you already know
        C#, Java, or TypeScript. You'll learn Rust by building <strong>Focus Forge</strong>, a real
        project, task, and note workbench, one chapter at a time: from tiny CLI labs to a desktop GUI.
      </p>
      <div class="cta">
        <a class="btn primary" href="https://github.com/soulwax/Learn2Rust">View on GitHub →</a>
        <a class="btn ghost" href="#start">Start the setup</a>
      </div>
    </section>

    <section id="start">
      <h2>Get started in three commands</h2>
      <p class="sub">You need Rust (via <a href="https://rustup.rs">rustup</a>) and, ideally, VS Code with the recommended extensions. Then clone and run the first lab from the repository root.</p>

      <div class="grid cols-3" style="margin-bottom:24px">
        <div class="card">
          <span class="step-n">1</span>
          <h3>Check your tools</h3>
          <p>Confirm the Rust toolchain is installed and on your PATH.</p>
        </div>
        <div class="card">
          <span class="step-n">2</span>
          <h3>Clone the repo</h3>
          <p>Grab the course and move into it. Everything runs from the root.</p>
        </div>
        <div class="card">
          <span class="step-n">3</span>
          <h3>Run the first lab</h3>
          <p>Build, test, and run Chapter 0 — your first working Rust program.</p>
        </div>
      </div>

<pre><span class="c"># 1 · check your tools</span>
<span class="p">rustc</span> --version
<span class="p">cargo</span> --version

<span class="c"># 2 · clone and enter the course</span>
<span class="p">git</span> clone https://github.com/soulwax/Learn2Rust.git
<span class="p">cd</span> Learn2Rust

<span class="c"># 3 · run the first lab (from the repo root)</span>
<span class="p">cargo</span> run -p ch00_setup
<span class="p">cargo</span> run -p ch00_setup -- <span class="s">YourName</span></pre>

      <div class="callout">
        <strong>Expected output:</strong> <code class="inline">Hello, Rust!</code> (or <code class="inline">Hello, YourName!</code>),
        followed by <code class="inline">Chapter 0 is alive. Cargo, VS Code, and you are talking.</code>
      </div>
    </section>

    <section id="verify">
      <h2>Verify the workspace</h2>
      <p class="sub">These four commands are the heartbeat of the course. Run them anytime — they check, test, format, and lint the entire Cargo workspace.</p>
<pre><span class="p">cargo</span> check                                              <span class="c"># fast type-check, whole workspace</span>
<span class="p">cargo</span> test                                               <span class="c"># run every test</span>
<span class="p">cargo</span> fmt --check                                        <span class="c"># verify formatting</span>
<span class="p">cargo</span> clippy --workspace --all-targets --all-features    <span class="c"># lint</span></pre>
      <p class="sub" style="margin-bottom:0">In VS Code, the same commands live under <strong>Terminal → Run Task</strong> (<code class="inline">cargo: check workspace</code>, <code class="inline">cargo: test workspace</code>, and friends).</p>
    </section>

    <section id="layout">
      <h2>How the workspace is laid out</h2>
      <p class="sub">One Cargo workspace is your home base. Real application code lives in <code class="inline">crates/</code>; small chapter experiments live in <code class="inline">labs/</code>. Every folder owns exactly one kind of thing.</p>
      <div class="tree">
<pre><span class="dir">Learn2Rust/</span>
├─ <span class="dir">crates/</span>
│  └─ <span class="dir">focus_forge_core/</span>   <span class="note"># the domain: projects, tasks, notes, JSON storage</span>
├─ <span class="dir">labs/</span>
│  └─ <span class="dir">ch00_setup/</span>        <span class="note"># your first runnable Rust lab</span>
├─ <span class="dir">chapters/</span>            <span class="note"># the teaching narrative</span>
├─ <span class="dir">assignments/</span>         <span class="note"># tasks, acceptance criteria, verification</span>
├─ <span class="dir">docs/</span>                <span class="note"># references, troubleshooting, decision records</span>
├─ <span class="dir">sample_data/</span>         <span class="note"># committed curriculum data</span>
└─ Cargo.toml           <span class="note"># the workspace manifest</span></pre>
      </div>
      <div class="grid cols-2" style="margin-top:20px">
        <div class="card">
          <h3>🧩 focus_forge_core</h3>
          <p>The heart of the app: domain types, validation, a single error type, and JSON persistence. It knows nothing about the CLI or GUI — they depend on it, never the other way around.</p>
        </div>
        <div class="card">
          <h3>🔬 labs/chXX</h3>
          <p>Tiny, focused packages — one Rust concept each. A safe sandbox to experiment, break things, and read the compiler's feedback. Run any lab with <code class="inline">cargo run -p &lt;name&gt;</code>.</p>
        </div>
      </div>
    </section>

    <section id="commands">
      <h2>Handy Cargo commands</h2>
      <p class="sub">Scope any command to one package with <code class="inline">-p</code>, or drill down to a single test by name.</p>
<pre><span class="p">cargo</span> run -p ch00_setup -- <span class="s">Soulwax</span>            <span class="c"># run a lab with an argument</span>
<span class="p">cargo</span> check  -p focus_forge_core               <span class="c"># check just one crate</span>
<span class="p">cargo</span> test   -p focus_forge_core               <span class="c"># test just one crate</span>
<span class="p">cargo</span> test   -p ch00_setup greeting_uses_provided_name   <span class="c"># one test by name</span></pre>
      <div class="callout">
        <strong>Coming next:</strong> a <code class="inline">focus_forge_cli</code> that drives the core crate —
        <code class="inline">project add / list / show</code>, <code class="inline">task add / done</code>, and
        <code class="inline">note add</code>. This page will grow a full command reference when it ships.
      </div>
    </section>

    <StatusSection />
  </div>
</main>

<Footer />
