<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import * as THREE from 'three';
  import type { NetworkInfo, ProcessInfo } from '$lib/types';

  let {
    baseColor = '#00d4ff',
    markerColor = '#ea00d9',
    commands = [],
    network = null,
    processes = [],
  }: {
    baseColor?: string;
    markerColor?: string;
    commands?: string[];
    network?: NetworkInfo | null;
    processes?: ProcessInfo[];
  } = $props();

  let container: HTMLDivElement;
  let animationId: number;
  let netIntensity = $state(0);

  const GROUPS = [
    { id: 'git', label: 'GIT', re: /\bgit\b/i },
    { id: 'container', label: 'DOCKER/K8S', re: /\b(docker|kubectl|helm)\b/i },
    { id: 'package', label: 'PKG', re: /\b(npm|pnpm|yarn|pip|brew|cargo)\b/i },
    { id: 'network', label: 'NET', re: /\b(curl|wget|ssh|scp|nc|nmap|ping)\b/i },
    { id: 'system', label: 'SYS', re: /\b(top|htop|ps|kill|lsof|df|du|free|vm_stat)\b/i },
  ];

  function computeHeatmap(items: string[]): { label: string; count: number; ratio: number }[] {
    const counts = GROUPS.map(g => ({
      label: g.label,
      count: items.filter(c => g.re.test(c)).length,
    }));
    const max = Math.max(1, ...counts.map(c => c.count));
    return counts.map(c => ({ ...c, ratio: c.count / max }));
  }

  const commandHeatmap = $derived(computeHeatmap(commands.slice(0, 80)));

  const radarInterfaces = $derived(
    (network?.interfaces || [])
      .map((i) => ({ name: i.name, traffic: i.received + i.transmitted }))
      .sort((a, b) => b.traffic - a.traffic)
      .slice(0, 4)
  );

  function computeSecurityAlerts(
    cmdItems: string[],
    procItems: ProcessInfo[]
  ): { severity: 'low' | 'medium' | 'high'; text: string }[] {
    const alerts: { severity: 'low' | 'medium' | 'high'; text: string }[] = [];
    const recent = cmdItems.slice(0, 20).join('\n');
    if (/\b(rm\s+-rf\s+\/|mkfs|dd\s+if=|chmod\s+777)\b/i.test(recent)) {
      alerts.push({ severity: 'high', text: 'Potential destructive command pattern detected.' });
    }
    if (/\b(nmap|nc\s+-l|tcpdump|wireshark)\b/i.test(recent)) {
      alerts.push({ severity: 'medium', text: 'Network probing/sniffing command observed.' });
    }
    const topCpu = [...procItems].sort((a, b) => b.cpu_usage - a.cpu_usage)[0];
    if (topCpu && topCpu.cpu_usage > 85) {
      alerts.push({ severity: 'medium', text: `High CPU process: ${topCpu.name} (${topCpu.cpu_usage.toFixed(0)}%).` });
    }
    if (alerts.length === 0) alerts.push({ severity: 'low', text: 'No immediate security anomalies.' });
    return alerts.slice(0, 3);
  }

  const securityAlerts = $derived(computeSecurityAlerts(commands, processes));

  function formatMB(bytes: number): string {
    return (bytes / 1024 / 1024).toFixed(1);
  }

  $effect(() => {
    const total = (network?.total_received || 0) + (network?.total_transmitted || 0);
    // Smooth logarithmic mapping for subtle animation scaling.
    netIntensity = Math.min(1, Math.log10(total + 1) / 11);
  });

  onMount(() => {
    const width = container.offsetWidth;
    const height = container.offsetHeight;

    const scene = new THREE.Scene();
    const camera = new THREE.PerspectiveCamera(45, width / height, 0.1, 1000);
    camera.position.z = 2.5;

    const renderer = new THREE.WebGLRenderer({ alpha: true, antialias: true });
    renderer.setSize(width, height);
    renderer.setPixelRatio(window.devicePixelRatio);
    container.appendChild(renderer.domElement);

    const geometry = new THREE.SphereGeometry(1, 32, 24);
    const material = new THREE.MeshBasicMaterial({
      color: new THREE.Color(baseColor),
      wireframe: true,
      transparent: true,
      opacity: 0.3,
    });
    const globe = new THREE.Mesh(geometry, material);
    scene.add(globe);

    const innerGeo = new THREE.SphereGeometry(0.98, 32, 24);
    const innerMat = new THREE.MeshBasicMaterial({
      color: new THREE.Color(baseColor),
      transparent: true,
      opacity: 0.05,
    });
    scene.add(new THREE.Mesh(innerGeo, innerMat));

    const pointsGeo = new THREE.BufferGeometry();
    const pointPositions: number[] = [];
    for (let i = 0; i < 200; i++) {
      const phi = Math.acos(-1 + (2 * i) / 200);
      const theta = Math.sqrt(200 * Math.PI) * phi;
      const x = Math.cos(theta) * Math.sin(phi) * 1.01;
      const y = Math.sin(theta) * Math.sin(phi) * 1.01;
      const z = Math.cos(phi) * 1.01;
      pointPositions.push(x, y, z);
    }
    pointsGeo.setAttribute('position', new THREE.Float32BufferAttribute(pointPositions, 3));
    const pointsMat = new THREE.PointsMaterial({
      color: new THREE.Color(markerColor),
      size: 0.02,
      transparent: true,
      opacity: 0.8,
    });
    const points = new THREE.Points(pointsGeo, pointsMat);
    scene.add(points);

    const ringGeo = new THREE.RingGeometry(1.2, 1.22, 64);
    const ringMat = new THREE.MeshBasicMaterial({
      color: new THREE.Color(baseColor),
      side: THREE.DoubleSide,
      transparent: true,
      opacity: 0.2,
    });
    const ring = new THREE.Mesh(ringGeo, ringMat);
    ring.rotation.x = Math.PI / 2.5;
    scene.add(ring);

    function animate() {
      animationId = requestAnimationFrame(animate);
      globe.rotation.y += 0.003 + netIntensity * 0.003;
      points.rotation.y += 0.003 + netIntensity * 0.003;
      ring.rotation.z += 0.001 + netIntensity * 0.002;
      ring.scale.setScalar(1 + netIntensity * 0.07);
      pointsMat.opacity = 0.35 + netIntensity * 0.65;
      renderer.render(scene, camera);
    }
    animate();

    const resizeObserver = new ResizeObserver(() => {
      const w = container.offsetWidth;
      const h = container.offsetHeight;
      camera.aspect = w / h;
      camera.updateProjectionMatrix();
      renderer.setSize(w, h);
    });
    resizeObserver.observe(container);

    return () => {
      resizeObserver.disconnect();
    };
  });

  onDestroy(() => {
    if (animationId) cancelAnimationFrame(animationId);
  });
</script>

<div class="w-full h-full min-h-[240px]" style="display:flex;flex-direction:column;gap:6px;">
  <div bind:this={container} class="w-full" style="height:150px;"></div>

  <div style="display:grid;grid-template-columns:1fr 1fr;gap:6px;padding:0 4px 4px;">
    <div style="border:1px solid var(--color-border);padding:4px;">
      <div style="font-size:8px;letter-spacing:1px;opacity:0.6;margin-bottom:3px;">COMMAND HEATMAP</div>
      {#each commandHeatmap as item}
        <div style="display:flex;align-items:center;gap:4px;margin:2px 0;">
          <span style="font-size:8px;opacity:0.8;width:54px;">{item.label}</span>
          <div style="flex:1;height:4px;background:var(--color-bg-primary);">
            <div style="height:100%;width:{Math.round(item.ratio * 100)}%;background:var(--color-primary);"></div>
          </div>
          <span style="font-size:8px;opacity:0.8;width:14px;text-align:right;">{item.count}</span>
        </div>
      {/each}
    </div>

    <div style="border:1px solid var(--color-border);padding:4px;">
      <div style="font-size:8px;letter-spacing:1px;opacity:0.6;margin-bottom:3px;">NETWORK RADAR</div>
      {#if radarInterfaces.length === 0}
        <div style="font-size:8px;opacity:0.5;">No active interface data</div>
      {:else}
        {#each radarInterfaces as iface}
          <div style="display:flex;align-items:center;justify-content:space-between;font-size:8px;margin:2px 0;">
            <span style="max-width:68px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;">{iface.name}</span>
            <span style="color:var(--color-text-bright);">{(iface.traffic / 1024 / 1024).toFixed(1)} MB</span>
          </div>
        {/each}
      {/if}
      <div style="margin-top:4px;font-size:8px;opacity:0.7;">
        RX {formatMB(network?.total_received || 0)} MB ·
        TX {formatMB(network?.total_transmitted || 0)} MB
      </div>
    </div>
  </div>

  <div style="border-top:1px solid var(--color-border);padding:4px 8px 0;">
    <div style="font-size:8px;letter-spacing:1px;opacity:0.6;margin-bottom:2px;">SECURITY WATCH</div>
    {#each securityAlerts as alert}
      <div style="font-size:8px;margin:1px 0;color:{alert.severity === 'high' ? 'var(--color-error)' : alert.severity === 'medium' ? 'var(--color-warning)' : 'var(--color-success)'};">
        [{alert.severity.toUpperCase()}] {alert.text}
      </div>
    {/each}
  </div>
</div>
