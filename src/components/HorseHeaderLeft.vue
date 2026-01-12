<script setup lang="ts">
import type { Horse } from "../models/racecard";
import Transformers from '../utils/transformers';

const props = defineProps<{
    horse: Horse;
}>();
</script>

<template>
            <div class="container">
                <div class="top">
                    <div class="col1">
                        <div class="program-number color-accent-yellow">{{ props.horse.post_position }}</div>
                        <div class="odds color-accent-yellow">{{
                            Transformers.createOddsString(props.horse.morning_line_odds) }}</div>
                    </div>
                    <div class="col2">
                        <div class="name-claiming color-accent-yellow">
                            <div class="horse-name color-accent-green">{{
                                Transformers.capitalize(props.horse.horse_name) }}
                                <span class="color-accent-yellow">({{ props.horse.bris_run_style }} {{
                                    props.horse.quirin_speed_points }})</span>
                            </div>
                            <div v-if="props.horse.claiming_price_of_horse"
                                class="claiming-price color-accent-yellow">{{
                                    Transformers.createDollarString(props.horse.claiming_price_of_horse) }}</div>
                        </div>
                        <div class="owner color-accent-green">Own:
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
</template>

<style lang="scss" scoped>
    .container {            
        flex: 0 0 25%;

        display: grid;
        grid-template-columns: auto 1fr;
        grid-template-rows: auto auto;
        gap: 1.5rem;
        align-items: start;
    }

    .col1 {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
        align-items: center;
        grid-column: 1;
        grid-row: 1;

        .program-number {
            font-size: 3rem;
            font-weight: 700;
        }

        .odds {
            font-size: 1.4rem;
        }
    }

    .col2 {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
        justify-content: flex-start;
        grid-column: 2;
        grid-row: 1;
        min-width: 0;
    }

    .top {
        display: contents;
    }

    .program-number {
        font-size: 3rem;
        font-weight: 700;
    }

    .odds {
        font-size: 1.4rem;
    }

    .owner {
        font-size: 1.4rem;
    }

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

    .horse-name {
        font-size: 2rem;
        font-weight: 500;
    }

    .jockey-info {
        grid-column: 1 / -1;
        grid-row: 2;
        text-align: left;
        width: 100%;
    }
</style>