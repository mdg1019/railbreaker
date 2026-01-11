<script setup lang="ts">
import type { Horse } from "../models/racecard";
import Transformers from '../utils/transformers';

const props = defineProps<{
    horse: Horse,
    primePowerComparisons: Array<[number | string, string, string]>;
}>();
</script>

<template>
    <div class="container">
        <div class="left">
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
        <div class="right">
            <div>
                <span v-if="props.horse.bris_prime_power_rating !== null" class="prime-power color-accent-green">Prime
                    Power:
                    <span
                        :style="{ color: props.primePowerComparisons.find(entry => entry[0] === props.horse.post_position)?.[2] ?? 'inherit' }">{{
                            Transformers.formatOneDecimal(props.horse.bris_prime_power_rating) }}
                        <span v-if="props.primePowerComparisons.find(entry => entry[0] === props.horse.post_position)">
                            <span>
                                {{props.primePowerComparisons.find(entry => entry[0] ===
                                    props.horse.post_position)?.[1] ?? ''}}
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
</template>

<style lang="scss" scoped>
.container {
    flex: 0 0 30%;
    min-width: 160px;
    align-self: start;
    display: grid;
    grid-template-columns: 1fr 1fr;
    grid-auto-rows: auto;
}

.left {
    grid-column: 1 / 3;
    grid-row: 1;
    align-self: start;

    display: flex;
    flex-direction: column;
    font-size: 1.2rem;
    gap: 0.25rem;
}

.right {
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
</style>