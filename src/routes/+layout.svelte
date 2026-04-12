<script lang="ts">
  import '../app.css';
  import '@xterm/xterm/css/xterm.css';

  let { children } = $props();
</script>

<div style="width:100vw;height:100vh;overflow:hidden;background:var(--color-bg-primary);position:relative;">
  {@render children()}
</div>
