<script lang="ts">
  let time = $state(new Date());

  $effect(() => {
    const interval = setInterval(() => {
      time = new Date();
    }, 1000);
    return () => clearInterval(interval);
  });

  let hours = $derived(time.getHours().toString().padStart(2, '0'));
  let minutes = $derived(time.getMinutes().toString().padStart(2, '0'));
  let seconds = $derived(time.getSeconds().toString().padStart(2, '0'));
  let date = $derived(time.toLocaleDateString('en-US', { weekday: 'short', month: 'short', day: 'numeric' }));
</script>

<div class="text-center py-3 sci-fi-panel">
  <div class="text-2xl font-bold glow-text tracking-[6px] font-mono">
    {hours}<span class="animate-pulse">:</span>{minutes}<span class="text-[var(--color-text)] text-sm">:{seconds}</span>
  </div>
  <div class="text-[10px] text-[var(--color-text)] tracking-[3px] mt-1 opacity-70">
    {date.toUpperCase()}
  </div>
</div>
