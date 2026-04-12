<script lang="ts">
  let { direction = 'horizontal', children }: {
    direction?: 'horizontal' | 'vertical';
    children: any;
  } = $props();

  let ratio = $state(50);
  let dragging = $state(false);
  let container: HTMLDivElement;

  function onMouseDown() {
    dragging = true;
  }

  function onMouseMove(e: MouseEvent) {
    if (!dragging || !container) return;
    const rect = container.getBoundingClientRect();
    if (direction === 'horizontal') {
      ratio = ((e.clientX - rect.left) / rect.width) * 100;
    } else {
      ratio = ((e.clientY - rect.top) / rect.height) * 100;
    }
    ratio = Math.max(20, Math.min(80, ratio));
  }

  function onMouseUp() {
    dragging = false;
  }
</script>

<svelte:window onmousemove={onMouseMove} onmouseup={onMouseUp} />

<div
  bind:this={container}
  class="w-full h-full flex"
  class:flex-row={direction === 'horizontal'}
  class:flex-col={direction === 'vertical'}
>
  <div style="{direction === 'horizontal' ? 'width' : 'height'}: {ratio}%;" class="overflow-hidden">
    {@render children()}
  </div>

  <div
    class="flex-shrink-0 bg-[var(--color-border)] hover:bg-[var(--color-primary)] transition-colors duration-200 cursor-{direction === 'horizontal' ? 'col' : 'row'}-resize z-10"
    class:w-1={direction === 'horizontal'}
    class:h-1={direction === 'vertical'}
    onmousedown={onMouseDown}
    role="separator"
    tabindex="-1"
  ></div>

  <div style="{direction === 'horizontal' ? 'width' : 'height'}: {100 - ratio}%;" class="overflow-hidden flex-1">
    <!-- Second pane rendered by parent -->
  </div>
</div>
