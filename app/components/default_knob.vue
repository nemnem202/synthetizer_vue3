<template>
  <svg
    width="40"
    height="40"
    viewBox="0 0 335 335"
    fill="none"
    xmlns="http://www.w3.org/2000/svg"
    :style="{ transform: `rotate(${(modelValue / 100) * 300 - 30}deg)` }"
  >
    <circle cx="167.5" cy="167.5" r="167" fill="#F1F1F1" stroke="url(#paint0_linear_2003_2)" />
    <g filter="url(#filter0_i_2003_2)">
      <circle cx="65" cy="226" r="15" fill="#FE621B" />
    </g>
    <defs>
      <filter
        id="filter0_i_2003_2"
        x="50"
        y="211"
        width="30"
        height="30"
        filterUnits="userSpaceOnUse"
        color-interpolation-filters="sRGB"
      >
        <feFlood flood-opacity="0" result="BackgroundImageFix" />
        <feBlend mode="normal" in="SourceGraphic" in2="BackgroundImageFix" result="shape" />
        <feColorMatrix
          in="SourceAlpha"
          type="matrix"
          values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0"
          result="hardAlpha"
        />
        <feMorphology
          radius="1"
          operator="erode"
          in="SourceAlpha"
          result="effect1_innerShadow_2003_2"
        />
        <feOffset />
        <feGaussianBlur stdDeviation="4.25" />
        <feComposite in2="hardAlpha" operator="arithmetic" k2="-1" k3="1" />
        <feColorMatrix type="matrix" values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0.15 0" />
        <feBlend mode="normal" in2="shape" result="effect1_innerShadow_2003_2" />
      </filter>
      <linearGradient
        id="paint0_linear_2003_2"
        x1="73"
        y1="30.5"
        x2="278"
        y2="299"
        gradientUnits="userSpaceOnUse"
      >
        <stop stop-color="white" />
        <stop offset="0.755" stop-color="#CBCBCB" />
      </linearGradient>
    </defs>
  </svg>
</template>

<script setup lang="ts">
const modelValue = defineModel<number>({ required: true });
</script>
