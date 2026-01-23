<script setup lang="ts">
const props = withDefaults(defineProps<{
    raceNumber: number,
    horseNumber: number,
    content: string,
    print: boolean;
}>(), {
    print: false,
});
</script>

<template>
    <div class="container font-small">
        <div class="label">Notes:</div>
        <textarea
            class="note-textarea font-small"
            rows="5"
            cols="40"
            :value="content"
        ></textarea>
    </div>
</template>

<style lang="scss" scoped>
.container {
    font-family: "MGSansCondensed";
    display: flex;
    flex-direction: row;
    gap: 0.5rem;
    margin-top: 1rem;
    width: 100%;
    box-sizing: border-box;
}

.label {
    color: var(--accent-yellow);
    margin-top: 0.75rem;
}

.note-textarea {
    flex: 1 1 auto;
    width: 100%;
    min-width: 0;
    padding: 0.5rem;
    background-color: var(--bg);
    color: var(--accent-green);
}
</style>
