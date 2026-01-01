<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  bars?: number
  width?: number
  height?: number
  color?: string
}

const props = withDefaults(defineProps<Props>(), {
  bars: 4,
  width: 55,
  height: 80,
  color: 'currentColor'
})

const barStyle = computed(() => ({
  width: `${props.width}px`,
  height: `${props.height}px`,
  color: props.color
}))
</script>

<template>
  <div class="equalizer" :style="barStyle">
    <span
      v-for="i in bars"
      :key="i"
      class="bar"
      :style="{ animationDelay: `${i * 0.15}s` }"
    />
  </div>
</template>

<style scoped>
.equalizer {
  display: flex;
  align-items: flex-end;
  gap: 8%;
}

.bar {
  flex: 1;
  height: 100%;
  background: currentColor;
  border-radius: 6%;
  transform-origin: bottom;
  animation: bounce 1.4s infinite ease-in-out;
}

.bar:nth-child(1) { animation-duration: 1.8s; }
.bar:nth-child(2) { animation-duration: 1.2s; }
.bar:nth-child(3) { animation-duration: 1.5s; }
.bar:nth-child(4) { animation-duration: 1.1s; }

@keyframes bounce {
  0%   { transform: scaleY(0.2); }
  20%  { transform: scaleY(0.9); }
  40%  { transform: scaleY(0.4); }
  60%  { transform: scaleY(1); }
  80%  { transform: scaleY(0.5); }
  100% { transform: scaleY(0.2); }
}
</style>
