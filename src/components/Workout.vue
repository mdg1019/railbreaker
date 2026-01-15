<script setup lang="ts">
import type { Workout } from "../models/racecard";

const props = defineProps<{
    workout: Workout;
}>();
</script>

<template>
    <div class="container">
  
            <div>1</div>
            <div>2</div>
            <div>3</div>
            <div>4</div>
            <div>5</div>
            <div>6</div>
            <div>7</div>

    </div>
</template>

<style scoped>
.container {
    display: grid;
    grid-template-columns: 4rem 4rem 4rem 4rem 4rem 4fr 4rem;
    gap: 0.25rem 0.5rem;
    font-size: 1.2rem;
}
</style>
