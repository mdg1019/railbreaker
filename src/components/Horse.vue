<script setup lang="ts">
import type { Horse } from "../models/racecard";
import HorseRecord from "./HorseRecord.vue";
import Panel from "./Panel.vue";
import Transformers from "../utils/transformers";

const props = defineProps<{ horse: Horse, primePowerComparisons: Array<[number | string, string, string]> }>();
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
                                <div v-if="props.horse.claiming_price_of_horse"
                                    class="claiming-price color-accent-yellow">{{
                                        Transformers.createDollarString(props.horse.claiming_price_of_horse) }}</div>
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

                <div class="horse-header-center">
                    <div class="horse-header-center-left">
                        <div class="color-accent-yellow">{{ Transformers.capitalizeWords(props.horse.horses_color) }}.
                                {{ props.horse.sex.toLowerCase() }}.
                                {{ Transformers.createAgeString(props.horse.horses_foaling_month, props.horse.year_of_birth) }}
                        </div>
                        <div>
                            <span class="color-accent-green">Sire: </span>
                            <span class="color-accent-yellow">{{
                                Transformers.capitalizeWords(props.horse.sire) }}</span>
                            <span class="color-accent-yellow"> ({{ Transformers.capitalizeWords(props.horse.sires_sire) }})</span>
                        </div>
                        <div>
                            <span class="color-accent-green">Brdr: </span>
                            <span class="color-accent-yellow">{{
                                Transformers.capitalizeWords(props.horse.breeder) }}</span>
                            <span class="color-accent-yellow"> ({{ props.horse.state_country_where_bred }})</span>
                        </div> 

                    <div class="trainer-info">
                        <span class="color-accent-green">Trnr: </span>
                        <span class="trainer color-accent-yellow">{{
                            Transformers.capitalizeWords(props.horse.todays_trainer) }}</span>
                        <span class="trainer-meet color-accent-yellow">
                            ({{ props.horse.trainer_starts }} {{ props.horse.trainer_wins }}-{{ props.horse.trainer_places
                            }}-{{ props.horse.trainer_shows }} {{
                                Transformers.createPercentageString(props.horse.trainer_wins, props.horse.trainer_starts) }})
                        </span>
                        <span class="trainer-previous-year color-accent-yellow">
                            {{ props.horse.current_year_record_year }}:
                            ({{ props.horse.trainer_wins_previous_year }}/{{ props.horse.trainer_starts_previous_year }}
                            {{ Transformers.createPercentageString(props.horse.trainer_wins_previous_year,
                                props.horse.trainer_starts_previous_year) }})
                        </span>
                    </div>
                    </div>
                    <div class="horse-header-center-right">
                        <div>
                            <span v-if="props.horse.bris_prime_power_rating !== null" class="prime-power color-accent-green">Prime Power: 
                                <span :style="{ color: props.primePowerComparisons.find(entry => entry[0] === props.horse.post_position)?.[2] ?? 'inherit' }">{{ Transformers.formatOneDecimal(props.horse.bris_prime_power_rating) }}
                                <span v-if="props.primePowerComparisons.find(entry => entry[0] === props.horse.post_position)">
                                    <span>
                                        {{ props.primePowerComparisons.find(entry => entry[0] === props.horse.post_position)?.[1] ?? '' }}
                                    </span>
                                </span>
                                </span>
                            </span>
                        </div>
                        <div class="blinkers color-accent-yellow">
                            <span v-show="props.horse.equipment_change === 1">Blinkers on</span>
                            <span v-show="props.horse.equipment_change === 2">Blinkers off</span>
                        </div>
                        <div>
                            <span>
                                <span v-if="props.horse.todays_medication_new === 1 
                                || props.horse.todays_medication_new === 3
                                || props.horse.todays_medication_new === 4
                                || props.horse.todays_medication_new === 5" 
                                    :style="{ color: props.horse.todays_medication_new === 4 || props.horse.todays_medication_new === 5 ? 'var(--accent-red)' : 'var(--accent-green)' }">L</span>
                                <span v-if="props.horse.todays_medication_new === 2
                                || props.horse.todays_medication_new === 3
                                || props.horse.todays_medication_new === 5" class="color-accent-green">B</span>
                            </span>
                            <span class="color-accent-yellow">{{ props.horse.weight }}</span>
                        </div>
                    </div>
                </div>

            <div class="horse-header-right">
                    <HorseRecord
                        :label="'Life:'"
                        :starts="props.horse.lifetime_record_starts ?? 0"
                        :wins="props.horse.lifetime_record_wins ?? 0"
                        :places="props.horse.lifetime_record_places ?? 0"
                        :shows="props.horse.lifetime_record_shows ?? 0"
                        :earnings="props.horse.lifetime_record_earnings ?? 0"
                        :speed-rating="props.horse.best_bris_speed_life ?? 0"
                        :pedigree="''"
                        :has-border="true"
                        class="life">
                    </HorseRecord>
                    <HorseRecord
                        :label="'Fst'"
                        :starts="props.horse.starts_fast_dirt ?? 0"
                        :wins="props.horse.wins_fast_dirt ?? 0"
                        :places="props.horse.places_fast_dirt ?? 0"
                        :shows="props.horse.shows_fast_dirt ?? 0"
                        :earnings="props.horse.earnings_fast_dirt ?? 0"
                        :speed-rating="props.horse.best_bris_speed_fast_track ?? 0"
                        :pedigree="props.horse.bris_dirt_pedigree_rating ?? ''"
                        :has-border="false"
                        class="fst">
                    </HorseRecord>                    
                    <HorseRecord
                        :label="props.horse.current_year_record_year?.toString() ?? ''"
                        :starts="props.horse.current_year_record_starts ?? 0"
                        :wins="props.horse.current_year_record_wins ?? 0"
                        :places="props.horse.current_year_record_places ?? 0"
                        :shows="props.horse.current_year_record_shows ?? 0"
                        :earnings="props.horse.current_year_record_earnings ?? 0"
                        :speed-rating="props.horse.best_bris_speed_most_recent_year ?? 0"
                        :pedigree="''"
                        :has-border="false"
                        class="this-year">
                    </HorseRecord>
                </div>

                <div class="horse-header-right-right">
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
    gap: 1rem;
    align-items: center;

    &-left {
        flex: 0 0 25%;

        display: grid;
        grid-template-columns: auto 1fr;
        grid-template-rows: auto auto;
        gap: 1.5rem;
        align-items: start;

        &-top {
            display: contents;
        }
    }

    &-center{
        flex: 0 0 30%;
        min-width: 160px;
        align-self: start;
        display: grid;
        grid-template-columns: 1fr 1fr;
        grid-auto-rows: auto;

        &-left {
            grid-column: 1 / 3;
            grid-row: 1;
            align-self: start;

            display: flex;
            flex-direction: column;
            font-size: 1.2rem;
            gap: 0.25rem;
        }

        &-right {
            grid-column: 1 / 3;
            grid-row: 1;
            z-index: 1;
            justify-self: end;
            text-align: right;
            align-self: start;
                        display: flex;
                        flex-direction: column;
                        gap: 0.25rem;
                        align-items: flex-end;
        }

        .blinkers {
            display: block;
            text-align: right;
            justify-self: end;
            min-width: 2rem;
            min-height: 1rem;
            visibility: visible;
        }
    }

    &-right {
        flex: 0 0 auto;

        align-self: start;
        display: grid;
        grid-template-columns: auto auto;
        grid-template-rows: auto auto;
        gap: 0;

        &-life {
            grid-column: 1;
            grid-row: 1;
        }

        &-fst {
            grid-column: 2;
            grid-row: 1;
        }

        &-this-year {
            grid-column: 1;
            grid-row: 2;
        }
    }

    .header-col1 {
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

    .header-col2 {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
        justify-content: flex-start;
        grid-column: 2;
        grid-row: 1;
        min-width: 0;

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
    }

    .jockey-info {
        grid-column: 1 / -1;
        grid-row: 2;
        text-align: left;
        width: 100%;
    }
}
</style>
