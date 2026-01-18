<script setup lang="ts">
    const props = withDefaults(defineProps<{
        print?: boolean;
    }>(), {
        print: false,
    });
</script>

<template>
    <div class="panel" :class="{ 'print': props.print }">
        <slot />
    </div>
</template>

<style scoped lang="scss">
.panel {
    box-sizing: border-box;
    margin-top: 1rem;
    margin-bottom: 1rem;
    padding: 1rem;
    border: 2px solid var(--ubuntu-blue);
    border-radius: 6px;
    background: var(--background-light);
}

.panel.print {
    box-sizing: border-box;
    margin-top: .5rem;
    margin-bottom: .5rem;
    border: none;
    border-radius: 0;
    background: transparent;
}
</style>
