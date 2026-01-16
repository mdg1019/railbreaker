<script setup lang="ts">
import type { Horse } from "../../../models/racecard";
import Transformers from '../../../utils/transformers';

const props = defineProps<{
    horse: Horse;
}>();
</script>

<template>
            <div class="container">
                <div class="top">
                    <div class="col1">
                        <div class="program-number color-accent-yellow">{{ props.horse.postPosition }}</div>
                        <div class="odds color-accent-yellow">{{
                            Transformers.createOddsString(props.horse.morningLineOdds) }}</div>
                    </div>
                    <div class="col2">
                        <div class="name-claiming color-accent-yellow">
                            <div class="horse-name color-accent-green">{{
                                Transformers.capitalize(props.horse.horseName) }}
                                <span class="color-accent-yellow">({{ props.horse.brisRunStyle }} {{
                                    props.horse.quirinSpeedPoints }})</span>
                            </div>
                            <div v-if="props.horse.claimingPriceOfHorse"
                                class="claiming-price color-accent-yellow">{{
                                    Transformers.createDollarString(props.horse.claimingPriceOfHorse) }}</div>
                        </div>
                        <div class="owner color-accent-green">Own:
                            <span class="color-accent-yellow">{{
                                Transformers.capitalizeFullName(props.horse.todaysOwner) }}</span>
                        </div>
                        <div class="ownersSilks color-accent-green">{{ props.horse.ownersSilks }}</div>
                    </div>
                </div>

                <div class="jockey-info">
                    <span class="jockey color-accent-yellow">{{
                        Transformers.capitalizeWords(props.horse.todaysJockey) }}</span>
                    <span class="jockey-meet color-accent-yellow">
                        ({{ props.horse.jockeyStarts }} {{ props.horse.jockeyWins }}-{{ props.horse.jockeyPlaces
                        }}-{{ props.horse.jockeyShows }} {{
                            Transformers.createPercentageString(props.horse.jockeyWins, props.horse.jockeyStarts) }})
                    </span>
                    <span class="jock-previous-year color-accent-yellow">
                        {{ props.horse.currentYearRecordYear }}:
                        ({{ props.horse.jockeyWinsPreviousYear }}/{{ props.horse.jockeyStartsPreviousYear }}
                        {{ Transformers.createPercentageString(props.horse.jockeyWinsPreviousYear,
                            props.horse.jockeyStartsPreviousYear) }})
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

    .ownersSilks {
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