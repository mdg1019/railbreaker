<script setup lang="ts">
import { nextTick, onBeforeUnmount, ref, watch } from 'vue'

const props = defineProps<{
  modelValue: boolean
  title?: string
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void
}>()

const dialogRef = ref<HTMLElement | null>(null)
let previouslyFocused: HTMLElement | null = null

function close() {
  emit('update:modelValue', false)
}

function isEscapeKey(e: KeyboardEvent) {
  return e.key === 'Escape' || e.code === 'Escape'
}

function getFocusableElements(container: HTMLElement): HTMLElement[] {
  const selectors = [
    'a[href]',
    'button:not([disabled])',
    'input:not([disabled])',
    'select:not([disabled])',
    'textarea:not([disabled])',
    '[tabindex]:not([tabindex="-1"])',
    '[contenteditable="true"]',
  ]

  const nodes = Array.from(container.querySelectorAll<HTMLElement>(selectors.join(',')))
  return nodes.filter((el) => {
    if (el.hasAttribute('disabled')) return false
    if (el.getAttribute('aria-hidden') === 'true') return false

    const style = window.getComputedStyle(el)
    if (style.visibility === 'hidden' || style.display === 'none') return false

    // If the element is not in layout, it can't be focused by the user.
    if (el.offsetParent === null && style.position !== 'fixed') return false
    return true
  })
}

async function focusInitialElement() {
  await nextTick()
  const dialog = dialogRef.value
  if (!dialog) return

  const autofocus = dialog.querySelector<HTMLElement>('[autofocus]')
  if (autofocus) {
    autofocus.focus()
    return
  }

  const focusables = getFocusableElements(dialog)
  if (focusables.length > 0) {
    focusables[0].focus()
  } else {
    dialog.focus()
  }
}

async function restorePreviousFocus() {
  const toRestore = previouslyFocused
  previouslyFocused = null
  await nextTick()
  toRestore?.focus?.()
}

function trapTabKey(e: KeyboardEvent) {
  if (e.key !== 'Tab') return

  const dialog = dialogRef.value
  if (!dialog) return

  const focusables = getFocusableElements(dialog)
  const active = document.activeElement as HTMLElement | null

  if (focusables.length === 0) {
    e.preventDefault()
    dialog.focus()
    return
  }

  const first = focusables[0]
  const last = focusables[focusables.length - 1]
  const isInside = !!active && dialog.contains(active)

  if (e.shiftKey) {
    if (!isInside || active === first) {
      e.preventDefault()
      last.focus()
    }
  } else {
    if (!isInside || active === last) {
      e.preventDefault()
      first.focus()
    }
  }
}

function handleKeydown(e: KeyboardEvent) {
  if (isEscapeKey(e)) {
    e.preventDefault()
    close()
    return
  }

  trapTabKey(e)
}

function handleFocusIn(e: FocusEvent) {
  const dialog = dialogRef.value
  const target = e.target as Node | null
  if (!dialog || !target) return

  if (!dialog.contains(target)) {
    void focusInitialElement()
  }
}

function handlePointerDown(e: PointerEvent) {
  const dialog = dialogRef.value
  const target = e.target as Node | null
  if (!dialog || !target) return

  // Clicking outside the modal shouldn't move focus away.
  if (!dialog.contains(target)) {
    e.preventDefault()

    const active = document.activeElement as HTMLElement | null
    if (active && dialog.contains(active)) {
      // Re-assert focus so it doesn't get cleared by the click.
      active.focus()
    } else {
      void focusInitialElement()
    }
  }
}

function addModalListeners() {
  window.addEventListener('keydown', handleKeydown, true)
  document.addEventListener('focusin', handleFocusIn, true)
  document.addEventListener('pointerdown', handlePointerDown, true)
}

function removeModalListeners() {
  window.removeEventListener('keydown', handleKeydown, true)
  document.removeEventListener('focusin', handleFocusIn, true)
  document.removeEventListener('pointerdown', handlePointerDown, true)
}

watch(
  () => props.modelValue,
  (isOpen) => {
    if (isOpen) {
      previouslyFocused = document.activeElement as HTMLElement | null
      addModalListeners()
      void focusInitialElement()
    } else {
      removeModalListeners()
      void restorePreviousFocus()
    }
  },
  { immediate: true },
)

onBeforeUnmount(() => {
  removeModalListeners()
})
</script>

<template>
  <Teleport to="body">
    <transition name="fade">
      <div v-if="modelValue" class="backdrop">
        <div ref="dialogRef" class="dialog" role="dialog" aria-modal="true" tabindex="-1">
          <div class="header-content-wrapper">
            <header class="titlebar">
              <span class="title">{{ title }}</span>
            </header>

            <section class="content">
              <slot />
            </section>
          </div>

          <footer class="actions" v-if="$slots.actions">
            <slot name="actions" />
          </footer>
        </div>
      </div>
    </transition>
  </Teleport>
</template>

<style lang="scss" scoped>
.backdrop {
  position: fixed;
  inset: 0;
  background: var(--modal-backdrop);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  outline: none;
}

.dialog {
  width: min(90vw, 520px);
  background: var(--modal-bg);
  color: var(--modal-fg);
  border: 2px solid var(--modal-border);
  border-radius: 10px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.6);
  overflow: hidden;
  font-family: system-ui, -apple-system, BlinkMacSystemFont, "Ubuntu", sans-serif;
}

.header-content-wrapper {
  display: flex;
  flex-direction: column;
  background: var(--modal-titlebar-bg);
  padding: 0.5rem 0;
}

.titlebar {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0.75rem 1rem;
}

.title {
  font-size: 2.5rem;
  font-weight: 600;
  text-align: center;
}

.content {
  padding: 1rem;
  font-size: 1.5rem;
  text-align: center;
}

.actions {
  display: flex;
  padding: 0;
  background: var(--modal-action-bg);
}

.actions :deep(button) {
  flex: 1 1 0;
  min-width: 0;
  appearance: none;
  background: transparent;
  color: inherit;
  border: 1px solid transparent;
  border-radius: 0;
  padding: 0.75rem 1rem;
  cursor: pointer;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  text-align: center;
  line-height: 1.1;
}

.actions :deep(button:hover) {
  background: var(--modal-action-hover-bg);
}

.actions :deep(button:focus-visible) {
  outline: none;
  box-shadow: none;
  border-color: var(--ubuntu-blue);
  border-top-left-radius: 0;
  border-top-right-radius: 0;
}

.actions :deep(button + button) {
  border-left: 1px solid var(--modal-border);
}

.actions :deep(button:first-child) {
  border-bottom-left-radius: 10px;
}

.actions :deep(button:last-child) {
  border-bottom-right-radius: 10px;
}

.dialog :deep(:focus-visible) {
  outline: none;
  box-shadow: 0 0 0 3px var(--ubuntu-blue-ring);
  border-radius: 6px;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.15s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
