<script setup lang="ts">
</script>

<template>
    <div class="panel">
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
</style>
