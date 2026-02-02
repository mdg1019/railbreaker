<script lang="ts">
import { defineComponent, PropType, nextTick, ref, watch } from 'vue';
import { openUrl } from '@tauri-apps/plugin-opener';
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
    linkText: {
      type: String as PropType<string>,
      default: ''
    },
    linkHref: {
      type: String as PropType<string>,
      default: ''
    },
    linkColor: {
      type: String as PropType<string>,
      default: ''
    },
    linkLabel: {
      type: String as PropType<string>,
      default: ''
    },
    link2Text: {
      type: String as PropType<string>,
      default: ''
    },
    link2Href: {
      type: String as PropType<string>,
      default: ''
    },
    link2Color: {
      type: String as PropType<string>,
      default: ''
    },
    link2Label: {
      type: String as PropType<string>,
      default: ''
    },
    okButtonColor: {
      type: String as PropType<string>,
      default: '--accent-yellow'
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

    const handleLinkClick = async (href: string, event: MouseEvent) => {
      if (!href) return
      event.preventDefault()
      await openUrl(href)
    }

    return { close, updateModelValue, okButton, handleLinkClick };
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
    <p class="message" :style="messageColor ? 'color: var(' + messageColor + ')' : undefined">
      {{ message }}
    </p>
    <p
      v-if="(linkText && linkHref) || (link2Text && link2Href)"
      class="message"
      :style="messageColor ? 'color: var(' + messageColor + ')' : undefined"
    >
      <span v-if="linkText && linkHref">{{ linkLabel }}</span>
      <a
        v-if="linkText && linkHref"
        class="link"
        :href="linkHref"
        :style="linkColor ? 'color: var(' + linkColor + ')' : undefined"
        @click="handleLinkClick(linkHref, $event)"
      >
        {{ linkText }}
      </a>
      <br v-if="linkText && linkHref && link2Text && link2Href" />
      <span v-if="link2Text && link2Href">{{ link2Label }}</span>
      <a
        v-if="link2Text && link2Href"
        class="link"
        :href="link2Href"
        :style="link2Color ? 'color: var(' + link2Color + ')' : undefined"
        @click="handleLinkClick(link2Href, $event)"
      >
        {{ link2Text }}
      </a>
    </p>

    <template #actions>
      <button
        ref="okButton"
        type="button"
        :style="okButtonColor ? 'color: var(' + okButtonColor + ')' : undefined"
        @click="close"
      >
        OK
      </button>
    </template>
  </ModalDialog>
</template>

<style scoped>
.message {
  white-space: pre-line;
}

.link {
  text-decoration: none;
  word-break: break-word;
}

.link:focus,
.link:focus-visible {
  outline: none;
  box-shadow: none;
}
</style>
