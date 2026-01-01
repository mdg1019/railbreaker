
<script setup lang="ts">
import { useGlobalStateStore } from "../stores/globalStateStore";

const globalStateStore = useGlobalStateStore();
</script>

<template>
    <section>
      <h2>Tracks</h2>
      <div v-if="globalStateStore.loading">Loading...</div>
      <div v-if="globalStateStore.error" style="color: red">{{ globalStateStore.error }}</div>
      <textarea
        rows="10"
        style="width: 100%; margin-top: 1em;"
        readonly
        :value="Array.from(globalStateStore.getTracks().entries())
          .sort(([a], [b]) => a.localeCompare(b))
          .map(([key, value]) => `${key}: ${value}`)
          .join('\n')"
      ></textarea>
    </section>
</template>

<style lang="scss" scoped>
.container {
  margin: 0 auto;
  padding: 2em;
  max-width: 600px;
}

section {
  background: #fff;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.08);
  padding: 2em;
  margin-top: 2em;
}

button {
  background: #249b73;
  color: #fff;
  border: none;
  border-radius: 4px;
  padding: 0.5em 1.5em;
  font-size: 1em;
  cursor: pointer;
  transition: background 0.2s;
}
button:disabled {
  background: #ccc;
  cursor: not-allowed;
}

textarea {
  font-family: monospace;
  font-size: 1em;
  background: #f6f6f6;
  border: 1px solid #ddd;
  border-radius: 4px;
  padding: 1em;
  margin-top: 1em;
  resize: vertical;
}

h2 {
  margin-bottom: 1em;
}
</style>

