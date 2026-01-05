<script setup lang="ts">
import { computed } from 'vue'
import type { Race } from '../models/racecard'
import Transformers from '../utils/transformers'

const props = defineProps<{ race: Race | null }>()

const tuple = computed(() => {
    if (!props.race) return ['', false]
    return Transformers.getRaceClassification(props.race)
})

const prefix = computed(() => tuple.value[0])
const race_classification = computed(() => tuple.value[1])
</script>

<template>
    <span class="container">
        <span class="prefix-adjust">{{ prefix }}</span>{{ race_classification }}
    </span>
</template>

<style scoped lang="scss">
.container {
    font-family: "MGSans", sans-serif;
}
.prefix-adjust {
    display: inline-block;
    transform: translateX(-0.10rem);
}
</style>
