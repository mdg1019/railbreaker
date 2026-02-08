<template>
  <span class="tooltip">
    <slot />
    <span v-if="props.text" class="tooltip-text" v-html="props.text"></span>
  </span>
</template>

<script setup lang="ts">
const props = defineProps<{
  text?: string
}>()
</script>

<style scoped>
.tooltip {
  position: relative;
  display: inline-block;
  cursor: default;
}

.tooltip-text {
  visibility: hidden;
  opacity: 0;
  position: absolute;
  bottom: calc(100% + 0.4rem);
  left: 50%;
  transform: translateX(-50%);
  background: rgba(0, 0, 0, 0.85);
  color: #fff;
  padding: 0.4rem 0.4rem;
  border: 2px solid var(--ubuntu-blue);
  border-radius: 4px;
  white-space: normal;
  width: 20rem;
  z-index: 1000;
  transition: opacity 0.12s ease-in-out;
  pointer-events: none;
  font-size: 1.4rem;
  text-align: left;
}

.tooltip:hover .tooltip-text,
.tooltip:focus-within .tooltip-text {
  visibility: visible;
  opacity: 1;
  pointer-events: auto;
}
</style>
