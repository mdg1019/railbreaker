<script setup lang="ts">
import { CardAnalysis } from "../../models/analysis";
import Panel from "../ui/Panel.vue";

const props = withDefaults(defineProps<{
    analysis: CardAnalysis,
    print: boolean;
}>(), {
    print: false,
});
</script>

<template>
    <Panel :print="props.print">
        <div class="contents">
            <div class="color-accent-yellow font-extra-large">Contextual Speed and Pace Model</div>
            <div class="color-accent-yellow font-large">{{ props.analysis.track }} - {{ props.analysis.date }}</div>
            <div></div>
            <div v-for="(race, idx) in props.analysis.races" :key="idx">
                <span>Race</span>{{ race.raceNumber }}
            </div>
        </div>
    </Panel>
</template>

<style scoped lang="scss">
.contents {
    display: flex;
    flex-direction: column;
}
</style>
