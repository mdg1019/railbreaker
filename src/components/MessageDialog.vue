<script lang="ts">
import { defineComponent, PropType } from 'vue';
import ModalDialog from './ModalDialog.vue';

export default defineComponent({
  name: 'MessageDialog',
  components: { ModalDialog },
  props: {
    modelValue: {
      type: Boolean as PropType<boolean>,
      required: true
    },
    title: {
      type: String as PropType<string>,
      default: ''
    },
    message: {
      type: String as PropType<string>,
      required: true
    }
  },
  emits: ['update:modelValue'],
  setup(props, { emit }) {
    const close = () => {
      emit('update:modelValue', false);
    };
    return { close };
  }
});
</script>

<template>
  <ModalDialog :modelValue="modelValue" @update:modelValue="$emit('update:modelValue', $event)">
    <div class="message-dialog">
      <div class="message-dialog-header">
        <div class="message-dialog-title" v-if="title">{{ title }}</div>
      </div>
      <div class="message-dialog-message">{{ message }}</div>
      <div class="message-dialog-actions">
        <button @click="close">OK</button>
      </div>
    </div>
  </ModalDialog>
</template>

<style scoped>
.message-dialog {
  display: flex;
  flex-direction: column;
  align-items: stretch;
}

.message-dialog-header {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 2.2rem;
}

.message-dialog-title {
  font-size: 2rem;
  font-weight: bold;
  color: var(--dialog-red);
  margin: 0 0 1rem 0;
  line-height: 1.1;
  text-align: center;
}

.message-dialog-message {
  margin-bottom: 1.5rem;
  text-align: center;
}

.message-dialog-actions {
  display: flex;
  justify-content: center;
}

.message-dialog-actions button {
  padding: 0.5rem 1.5rem;
  font-size: 1rem;
  border: none;
  border-radius: 4px;
  background: var(--dialog-purple);
  color: #fff;
  cursor: pointer;
  transition: background 0.2s;
  box-shadow: 0 2px 8px rgba(109, 40, 217, 0.08);
}

.message-dialog-actions button:hover {
  background: var(--dialog-purple-hover);
}
</style>
