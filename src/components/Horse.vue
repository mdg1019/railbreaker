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
                            <div class="name-claiming color-accent-yellow">
                                <div class="horse-name color-accent-green">{{
                                    Transformers.capitalize(props.horse.horse_name) }}
                                    <span class="color-accent-yellow">({{ props.horse.bris_run_style }} {{
                                        props.horse.quirin_speed_points }})</span>
                                </div>
                                <div v-if="props.horse.claiming_price_of_horse" class="claiming-price color-accent-yellow">{{ Transformers.createDollarString(props.horse.claiming_price_of_horse) }}</div>
                            </div>
                            <div class="color-accent-green">Own:
                                <span class="color-accent-yellow">{{
                                    Transformers.capitalizeFullName(props.horse.todays_owner) }}</span>
                            </div>
                            <div class="owners_silks color-accent-green">{{ props.horse.owners_silks }}</div>
                        </div>
                    </div>

                    <div class="jockey-info">
                        <span class="jockey color-accent-yellow">{{
                            Transformers.capitalizeWords(props.horse.todays_jockey) }}</span>
                        <span class="jockey-meet color-accent-yellow">
                            ({{ props.horse.jockey_starts }} {{ props.horse.jockey_wins }}-{{ props.horse.jockey_places
                            }}-{{ props.horse.jockey_shows }} {{
                                Transformers.createPercentageString(props.horse.jockey_wins, props.horse.jockey_starts) }})
                        </span>
                        <span class="jock-previous-year color-accent-yellow">
                            {{ props.horse.current_year_record_year }}:
                            ({{ props.horse.jockey_wins_previous_year }}/{{ props.horse.jockey_starts_previous_year }}
                            {{ Transformers.createPercentageString(props.horse.jockey_wins_previous_year,
                                props.horse.jockey_starts_previous_year) }})
                        </span>
                    </div>

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
        align-items: flex-start;

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
        align-items: center;
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

        .name-claiming {
            display: flex;
            flex-direction: row;
            gap: 1rem;
            align-items: center;
            width: 100%;
        }

        .claiming-price {
            margin-left: auto;
            text-align: right;
        }
    }

    .jockey-info {
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
