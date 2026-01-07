<script setup lang="ts">
import type { Horse } from "../models/racecard";
import Panel from "./Panel.vue";
import Transformers from "../utils/transformers";

const props = defineProps<{ horse: Horse }>();
</script>

<template>
    <Panel>
        <div class="horse">
            <div class="horse-header">
                <div class="header-col1">
                    <div class="program_number">{{ props.horse.post_position }}</div>
                    <div class="odds">{{ Transformers.createOddsString(props.horse.morning_line_odds) }}</div>
                </div>
                <div class="horse-name">{{ Transformers.capitalize(props.horse.horse_name) }}</div>
            </div>
            <div class="horse-body">
                <!-- additional horse info can go here -->
            </div>
        </div>
    </Panel>
</template>

<style scoped lang="scss">
.horse {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
}

.horse-header {
    display: flex;
    flex-direction: row;
    gap: 0.5rem;
    align-items: center;

    .header-col1 {
        display:flex;
        flex-direction: column;
        gap: 0.5rem;
    }
}

.program-number {
    font-weight: 700;
}

.horse-name {
    font-weight: 500;
}
</style>
