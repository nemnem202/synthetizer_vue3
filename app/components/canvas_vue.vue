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
import { ref, onMounted, watchEffect } from "vue";
import type { Curve } from "~/types/canvas";

const props = defineProps<{ curve: Curve }>();

const canvas_ref = ref<HTMLCanvasElement | null>(null);
const ctx = ref<CanvasRenderingContext2D | null>(null);

const canvas_width = ref(400);
const canvas_height = ref(200);
let world_height = 500;
let world_width = props.curve.points.length;

let scale_x = canvas_width.value / world_width;
let scale_y = canvas_height.value / world_height;
let offset_x = 0;
let offset_y = 0;

let max_scale = 5;
let min_scale_x = scale_x;
let min_scale_y = scale_y;

const clamp = (value: number, min: number, max: number) => Math.max(min, Math.min(max, value));

const clamp_camera = () => {
  const min_offset_x = canvas_width.value - world_width * scale_x;
  const min_offset_y = canvas_height.value - world_height * scale_y;
  offset_x = clamp(offset_x, min_offset_x, 0);
  offset_y = clamp(offset_y, min_offset_y, 0);
};

const world_to_screen = (x: number, y: number) => ({
  x: x * scale_x + offset_x,
  y: y * scale_y + offset_y,
});

const screen_to_world = (x: number, y: number) => ({
  x: (x - offset_x) / scale_x,
  y: (y - offset_y) / scale_y,
});

const draw = () => {
  if (!ctx.value || !canvas_ref.value) return;

  const context = ctx.value;
  context.setTransform(1, 0, 0, 1, 0, 0);
  context.clearRect(0, 0, canvas_ref.value.width, canvas_ref.value.height);

  context.translate(offset_x, offset_y);
  context.scale(scale_x, scale_y);

  context.beginPath();
  context.strokeStyle = "#fe621b1b";
  context.lineWidth = 0.05;
  for (let i = 0; i <= world_width; i += 10) {
    context.moveTo(i, 0);
    context.lineTo(i, world_height);
  }
  for (let i = 0; i <= world_height; i += 10) {
    context.moveTo(0, i);
    context.lineTo(world_width, i);
  }
  context.stroke();

  context.beginPath();
  context.strokeStyle = props.curve.color;
  context.lineWidth = 2 / scale_x;
  context.moveTo(0, props.curve.points[0] ? props.curve.points[0] * world_height : 0);

  const start_idx = Math.max(0, Math.floor(-offset_x / scale_x));
  const end_idx = Math.min(world_width, Math.ceil((canvas_width.value - offset_x) / scale_x));
  const step = Math.max(1, Math.floor(1 / scale_x));

  for (let i = start_idx; i < end_idx; i += step) {
    const p = props.curve.points[i];
    if (p == null) continue;
    context.lineTo(i, (p + 0.5) * world_height);
  }

  context.stroke();
};

const handle_mousedown = (event: MouseEvent) => {
  let last_x = event.clientX;
  let last_y = event.clientY;

  const on_mousemove = (ev: MouseEvent) => {
    offset_x += ev.clientX - last_x;
    offset_y += ev.clientY - last_y;
    last_x = ev.clientX;
    last_y = ev.clientY;
    clamp_camera();
    draw();
  };

  const on_mouseup = () => {
    window.removeEventListener("mousemove", on_mousemove);
    window.removeEventListener("mouseup", on_mouseup);
  };

  window.addEventListener("mousemove", on_mousemove);
  window.addEventListener("mouseup", on_mouseup);
};

const handle_wheel = (event: WheelEvent) => {
  if (!canvas_ref.value) return;

  const rect = canvas_ref.value.getBoundingClientRect();
  const mouse_world = screen_to_world(event.clientX - rect.left, event.clientY - rect.top);

  const zoom_factor = event.deltaY < 0 ? 1.1 : 0.9;
  if (event.ctrlKey) {
    scale_y = clamp(scale_y * zoom_factor, min_scale_y, max_scale);
  } else {
    scale_x = clamp(scale_x * zoom_factor, min_scale_x, max_scale);
  }

  const new_screen = world_to_screen(mouse_world.x, mouse_world.y);
  offset_x += event.clientX - rect.left - new_screen.x;
  offset_y += event.clientY - rect.top - new_screen.y;

  clamp_camera();
  draw();
};

onMounted(() => {
  if (!canvas_ref.value) return;
  ctx.value = canvas_ref.value.getContext("2d");
  draw();
});

watchEffect(() => {
 world_height = 500;
 world_width = props.curve.points.length;

 scale_x = canvas_width.value / world_width;
 scale_y = canvas_height.value / world_height;
 offset_x = 0;
 offset_y = 0;

 max_scale = 5;
 min_scale_x = scale_x;
 min_scale_y = scale_y;
  draw();
});
</script>
<style scoped>
.canvas-vue {
  background-color: black;
  cursor: grabbing;
}
</style>
