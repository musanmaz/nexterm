<script lang="ts">
  import { untrack } from 'svelte';

  let { data = [], maxPoints = 60, color = 'var(--color-chart-1)', height = 60, label = '' }: {
    data?: number[];
    maxPoints?: number;
    color?: string;
    height?: number;
    label?: string;
  } = $props();

  let canvas: HTMLCanvasElement;
  let points = $state<number[]>([]);

  function resolveColor(c: string): string {
    if (!c.startsWith('var(')) return c;
    try {
      const varName = c.replace(/var\(([^)]+)\)/, '$1').trim();
      const resolved = getComputedStyle(document.documentElement).getPropertyValue(varName).trim();
      return resolved || '#00d4ff';
    } catch {
      return '#00d4ff';
    }
  }

  function hexToRgba(hex: string, alpha: number): string {
    if (hex.startsWith('rgba')) return hex;
    if (hex.startsWith('rgb(')) {
      return hex.replace('rgb(', 'rgba(').replace(')', `, ${alpha})`);
    }
    hex = hex.replace('#', '');
    if (hex.length === 3) hex = hex[0]+hex[0]+hex[1]+hex[1]+hex[2]+hex[2];
    const r = parseInt(hex.substring(0, 2), 16);
    const g = parseInt(hex.substring(2, 4), 16);
    const b = parseInt(hex.substring(4, 6), 16);
    return `rgba(${r}, ${g}, ${b}, ${alpha})`;
  }

  $effect(() => {
    if (data.length > 0) {
      const latest = data[data.length - 1];
      const prev = untrack(() => points);
      points = [...prev.slice(-(maxPoints - 1)), latest];
    }
  });

  $effect(() => {
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    try {
      const w = canvas.width = canvas.offsetWidth * window.devicePixelRatio;
      const h = canvas.height = canvas.offsetHeight * window.devicePixelRatio;
      ctx.scale(window.devicePixelRatio, window.devicePixelRatio);

      const cw = canvas.offsetWidth;
      const ch = canvas.offsetHeight;

      ctx.clearRect(0, 0, cw, ch);

      ctx.strokeStyle = 'rgba(0, 212, 255, 0.07)';
      ctx.lineWidth = 0.5;
      for (let i = 0; i < 4; i++) {
        const y = (ch / 4) * i;
        ctx.beginPath();
        ctx.moveTo(0, y);
        ctx.lineTo(cw, y);
        ctx.stroke();
      }

      if (points.length < 2) return;

      const max = Math.max(...points, 1);
      const step = cw / (maxPoints - 1);
      const resolvedColor = resolveColor(color);

      const gradient = ctx.createLinearGradient(0, 0, 0, ch);
      gradient.addColorStop(0, hexToRgba(resolvedColor, 0.3));
      gradient.addColorStop(1, 'rgba(0, 0, 0, 0)');

      ctx.beginPath();
      ctx.moveTo(0, ch);
      for (let i = 0; i < points.length; i++) {
        const x = (maxPoints - points.length + i) * step;
        const y = ch - (points[i] / max) * (ch - 4);
        ctx.lineTo(x, y);
      }
      ctx.lineTo((maxPoints - 1) * step, ch);
      ctx.closePath();
      ctx.fillStyle = hexToRgba(resolvedColor, 0.1);
      ctx.fill();

      ctx.beginPath();
      for (let i = 0; i < points.length; i++) {
        const x = (maxPoints - points.length + i) * step;
        const y = ch - (points[i] / max) * (ch - 4);
        if (i === 0) ctx.moveTo(x, y);
        else ctx.lineTo(x, y);
      }
      ctx.strokeStyle = resolvedColor;
      ctx.lineWidth = 1.5;
      ctx.stroke();
    } catch {
      // Canvas rendering error - don't crash the component tree
    }
  });
</script>

<div style="width:100%;position:relative;height:{height}px;">
  {#if label}
    <span style="position:absolute;top:0;left:4px;font-size:10px;color:var(--color-text);opacity:0.6;z-index:10;">{label}</span>
  {/if}
  <canvas bind:this={canvas} style="width:100%;height:100%;display:block;"></canvas>
</div>
