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
    },
    titleColor: {
      type: String as PropType<string>,
      default: ''
    },
    messageColor: {
      type: String as PropType<string>,
      default: ''
    },
    okButtonColor: {
      type: String as PropType<string>,
      default: ''
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
    :title-color="titleColor"
    @update:modelValue="updateModelValue"
  >
    <p :style="messageColor ? 'color: var(' + messageColor + ')' : undefined">{{ message }}</p>

    <template #actions>
      <button
        ref="okButton"
        type="button"
        :style="okButtonColor ? 'background-color: var(' + okButtonColor + ')' : undefined"
        @click="close"
      >
        OK
      </button>
    </template>
  </ModalDialog>
</template>

<style scoped>

</style>
