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

const getBarStyle = (index: number) => {
  const durations = [1.8, 1.2, 1.5, 1.1, 1.6, 1.3, 1.7]
  const duration = durations[index % durations.length]
  return {
    'animation-delay': `${index * 0.15}s`,
    'animation-duration': `${duration}s`
  }
}
</script>

<template>
  <div class="equalizer" :style="{ ...barStyle, '--bar-count': props.bars }">
    <span
      v-for="i in bars"
      :key="i"
      class="bar"
      :style="getBarStyle(i - 1)"
    />
  </div>
</template>

<style scoped>

.equalizer {
  display: grid;
  align-items: flex-end;
  grid-template-columns: repeat(var(--bar-count), 1fr);
  gap: 8%;
  width: 100%;
  height: 100%;
}

.bar {
  min-width: 0;
  width: 100%;
  height: 100%;
  background: currentColor;
  border-radius: 6%;
  transform-origin: bottom;
  animation-name: bounce;
  animation-iteration-count: infinite;
  animation-timing-function: ease-in-out;
}

@keyframes bounce {
  0%   { transform: scaleY(0.2); }
  20%  { transform: scaleY(0.9); }
  40%  { transform: scaleY(0.4); }
  60%  { transform: scaleY(1); }
  80%  { transform: scaleY(0.5); }
  100% { transform: scaleY(0.2); }
}
</style>
