<script setup lang="ts">
import { Race } from '../../models/racecard'
import Transformers from '../../utils/transformers'

const props = withDefaults(defineProps<{ race: Race | null; prefixColor?: string }>(), {
    prefixColor: 'var(--accent-yellow)'
})

const raceClassification = Transformers.getRaceClassification(props.race!);
</script>

<template>
    <span class="container">
        <span class="prefix-adjust" :style="{ color: props.prefixColor }">{{ raceClassification.prefix }}</span>{{ raceClassification.classification }}
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
