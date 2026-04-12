<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import * as THREE from 'three';

  let { baseColor = '#00d4ff', markerColor = '#ea00d9' }: {
    baseColor?: string;
    markerColor?: string;
  } = $props();

  let container: HTMLDivElement;
  let animationId: number;

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
      globe.rotation.y += 0.003;
      points.rotation.y += 0.003;
      ring.rotation.z += 0.001;
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

<div bind:this={container} class="w-full h-full min-h-[180px]"></div>
