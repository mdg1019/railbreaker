<script lang="ts">
import { defineComponent, PropType, nextTick, ref, watch } from 'vue';
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
    const okButton = ref<HTMLButtonElement | null>(null)

    const close = () => {
      emit('update:modelValue', false);
    };

    const updateModelValue = (value: boolean) => {
      emit('update:modelValue', value);
    };

    watch(
      () => props.modelValue,
      async (isOpen) => {
        if (!isOpen) return
        await nextTick()
        okButton.value?.focus()
      },
      { immediate: true },
    )

    return { close, updateModelValue, okButton };
  }
});
</script>

<template>
  <ModalDialog
    :model-value="modelValue"
    :title="title"
    @update:modelValue="updateModelValue"
  >
    <p>{{ message }}</p>

    <template #actions>
      <button ref="okButton" type="button" @click="close">OK</button>
    </template>
  </ModalDialog>
</template>

<style scoped>

</style>
