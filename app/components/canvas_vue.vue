<template>
  <canvas
    ref="canvas_ref"
    :width="canvas_width"
    :height="canvas_height"
    @mousedown="handle_mousedown"
    @wheel.prevent="handle_wheel"
    class="canvas-vue"
  ></canvas>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import type { Curve } from "~/types/canvas";

// === Props ===
const props = defineProps<{ curve: Curve }>();

// === Refs canvas ===
const canvas_ref = ref<HTMLCanvasElement | null>(null);
const ctx = ref<CanvasRenderingContext2D | null>(null);

// === Canvas & world ===
const canvas_width = ref(400);
const canvas_height = ref(200);

let world_width = props.curve.points.length;

watchEffect(() => {
  world_width = props.curve.points.length;
  draw();
});
const world_height = 500;

// === Camera / zoom ===
let scale_x = canvas_width.value / world_width;
let scale_y = canvas_height.value / world_height;
let offset_x = 0;
let offset_y = 0;
const max_scale = 5;
let min_scale_x = canvas_width.value / world_width;
let min_scale_y = canvas_height.value / world_height;

// === Utils ===
function clamp(value: number, min: number, max: number) {
  return Math.max(min, Math.min(max, value));
}

function clamp_camera() {
  const min_offset_x = canvas_width.value - world_width * scale_x;
  const min_offset_y = canvas_height.value - world_height * scale_y;
  offset_x = clamp(offset_x, min_offset_x, 0);
  offset_y = clamp(offset_y, min_offset_y, 0);
}

// === Coordinate conversions ===
function world_to_screen(x: number, y: number) {
  return { x: x * scale_x + offset_x, y: y * scale_y + offset_y };
}
function screen_to_world(x: number, y: number) {
  return { x: (x - offset_x) / scale_x, y: (y - offset_y) / scale_y };
}

// === Draw ===
function draw() {
  if (!ctx.value || !canvas_ref.value) return;

  ctx.value.setTransform(1, 0, 0, 1, 0, 0);
  ctx.value.clearRect(0, 0, canvas_ref.value.width, canvas_ref.value.height);

  ctx.value.translate(offset_x, offset_y);
  ctx.value.scale(scale_x, scale_y);

  // grille
  ctx.value.beginPath();
  ctx.value.strokeStyle = "#fe621b1b";
  ctx.value.lineWidth = 0.05;
  for (let i = 0; i <= world_width; i += 10) {
    ctx.value.moveTo(i, 0);
    ctx.value.lineTo(i, world_height);
  }
  for (let i = 0; i <= world_height; i += 10) {
    ctx.value.moveTo(0, i);
    ctx.value.lineTo(world_width, i);
  }
  ctx.value.stroke();

  ctx.value.beginPath();
  ctx.value.strokeStyle = props.curve.color;
  ctx.value.lineWidth = 2 / scale_y;

  ctx.value.moveTo(0, props.curve.points[0] ? props.curve.points[0] * world_height : 0);

  const start_idx = Math.max(0, Math.floor(-offset_x / scale_x));
  const end_idx = Math.min(world_width, Math.ceil((canvas_width.value - offset_x) / scale_x));

  const step = Math.max(1, Math.floor(1 / scale_x)); // plus scale_x est petit, plus step est grand
  for (let i = start_idx; i < end_idx; i += step) {
    const p = props.curve.points[i];
    if (!p) continue;
    ctx.value.lineTo(i, (p + 0.5) * world_height);
  }

  ctx.value.stroke();
}

// === Mouse Drag ===
const handle_mousedown = (event: MouseEvent) => {
  let last_x = event.clientX;
  let last_y = event.clientY;

  function on_mousemove(ev: MouseEvent) {
    offset_x += ev.clientX - last_x;
    offset_y += ev.clientY - last_y;
    last_x = ev.clientX;
    last_y = ev.clientY;

    clamp_camera();
    draw();
  }

  function on_mouseup() {
    window.removeEventListener("mousemove", on_mousemove);
    window.removeEventListener("mouseup", on_mouseup);
  }

  window.addEventListener("mousemove", on_mousemove);
  window.addEventListener("mouseup", on_mouseup);
};

// === Wheel / Zoom ===
const handle_wheel = (event: WheelEvent) => {
  if (!canvas_ref.value) return;

  const rect = canvas_ref.value.getBoundingClientRect();
  const mouse_world = screen_to_world(event.clientX - rect.left, event.clientY - rect.top);

  const zoom = event.deltaY < 0 ? 1.1 : 0.9;
  if (event.ctrlKey) {
    // zoom vertical
    scale_y = clamp(scale_y * zoom, min_scale_y, max_scale);
  } else {
    // zoom horizontal
    scale_x = clamp(scale_x * zoom, min_scale_x, max_scale);
  }

  // garder le point sous la souris fixe
  const new_screen = world_to_screen(mouse_world.x, mouse_world.y);
  offset_x += event.clientX - rect.left - new_screen.x;
  offset_y += event.clientY - rect.top - new_screen.y;

  clamp_camera();
  draw();
};

// === Lifecycle ===
onMounted(() => {
  if (canvas_ref.value) {
    ctx.value = canvas_ref.value.getContext("2d");
    draw();
  }
});
</script>

<style scoped>
.canvas-vue {
  background-color: black;
  cursor: grabbing;
}
</style>
