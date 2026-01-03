<template>
  <div v-if="visible" class="modal-overlay" @click.self="close">
    <div class="modal-content">
      <button class="modal-cancel" @click="close" title="Cancel">&times;</button>
      <slot />
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, watch, PropType, onMounted, onUnmounted } from 'vue';

export default defineComponent({
  name: 'ModalDialog',
  props: {
    modelValue: {
      type: Boolean as PropType<boolean>,
      required: true
    }
  },
  emits: ['update:modelValue'],
  setup(props, { emit }) {
    const visible = ref(props.modelValue);

    watch(() => props.modelValue, (val) => {
      visible.value = val;
    });

    const close = () => {
      visible.value = false;
      emit('update:modelValue', false);
    };

    // Close modal on Escape key
    const handleKeydown = (event: KeyboardEvent) => {
      if (event.key === 'Escape' && visible.value) {
        close();
      }
    };

    onMounted(() => {
      window.addEventListener('keydown', handleKeydown);
    });
    onUnmounted(() => {
      window.removeEventListener('keydown', handleKeydown);
    });

    return { visible, close };
  }
});
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}
.modal-content {
  background: var(--bg);
  color: var(--fg);
  padding: 2rem;
  border-radius: 8px;
  min-width: 300px;
  max-width: 90vw;
  max-height: 90vh;
  overflow: auto;
  position: relative;
}
.modal-cancel {
  position: absolute;
  top: 0.5rem;
  right: 1rem;
  background: none;
  border: none;
  font-size: 3.0rem;
  color: var(--dialog-red);
  border-radius: 4px;
  cursor: pointer;
  z-index: 2;
  transition: background 0.2s;
}
.modal-cancel:hover {
  background: rgba(0,0,0,0.07);
}
</style>
