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

                <div class="horse-header-left">

                    <div class="horse-header-left-top">
                        <div class="header-col1">
                            <div class="program-number color-accent-yellow">{{ props.horse.post_position }}</div>
                            <div class="odds color-accent-yellow">{{
                                Transformers.createOddsString(props.horse.morning_line_odds) }}</div>
                        </div>
                        <div class="header-col2">
                            <div class="horse-name color-accent-green">{{
                                Transformers.capitalize(props.horse.horse_name) }}
                                <span class="color-accent-yellow">({{ props.horse.bris_run_style }} {{
                                    props.horse.quirin_speed_points }})</span>
                            </div>
                            <div class="color-accent-green">Own:
                                <span class="color-accent-yellow">{{
                                    Transformers.capitalizeFullName(props.horse.todays_owner) }}</span>
                            </div>
                            <div class="owners_silks color-accent-green">{{ props.horse.owners_silks }}</div>
                        </div>
                    </div>
                    <div class="jockey color-accent-yellow">{{ Transformers.capitalizeWords(props.horse.todays_jockey) }}</div>

                </div>

            </div>

            <div class="horse-body">
                <!-- additional horse info can go here -->
            </div>
        </div>
    </Panel>
</template>

<style scoped lang="scss">
.horse {
    font-family: "MGSansCondensed", sans-serif;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
}

.horse-header {
    display: flex;
    flex-direction: row;
    gap: 0.5rem;
    align-items: center;

    &-left {
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
        align-items: center;

        &-top {
            display: flex;
            flex-direction: row;
            gap: 2rem;
        }
    }

    .header-col1 {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .header-col2 {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
        justify-content: flex-start;
        align-self: flex-start;

        .owners_silks {
            font-size: 1.2rem;
        }
    }

    .jockey {
        align-self: flex-start;
        text-align: left;
        width: 100%;
    }
}

.program-number {
    font-size: 3rem;
    font-weight: 700;
}

.odds {
    font-size: 1.4rem;
}

.horse-name {
    font-size: 2rem;
    font-weight: 500;
}
</style>
