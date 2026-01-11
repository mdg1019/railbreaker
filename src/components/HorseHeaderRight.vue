<script setup lang="ts">
import type { Horse } from "../models/racecard";
import HorseRecord from "./HorseRecord.vue";

const props = defineProps<{
    horse: Horse;
}>();
</script>

<template>
    <div class="container">
        <HorseRecord :label="'Life:'" :starts="props.horse.lifetime_record_starts ?? 0"
            :wins="props.horse.lifetime_record_wins ?? 0" :places="props.horse.lifetime_record_places ?? 0"
            :shows="props.horse.lifetime_record_shows ?? 0" :earnings="props.horse.lifetime_record_earnings ?? 0"
            :speed-rating="props.horse.best_bris_speed_life ?? 0" :pedigree="''" :has-border="true" class="life">
        </HorseRecord>
        <HorseRecord :label="'Fst'" :starts="props.horse.starts_fast_dirt ?? 0" :wins="props.horse.wins_fast_dirt ?? 0"
            :places="props.horse.places_fast_dirt ?? 0" :shows="props.horse.shows_fast_dirt ?? 0"
            :earnings="props.horse.earnings_fast_dirt ?? 0" :speed-rating="props.horse.best_bris_speed_fast_track ?? 0"
            :pedigree="props.horse.bris_dirt_pedigree_rating ?? ''" :has-border="false" class="fst">
        </HorseRecord>
        <HorseRecord :label="props.horse.current_year_record_year?.toString() ?? ''"
            :starts="props.horse.current_year_record_starts ?? 0" :wins="props.horse.current_year_record_wins ?? 0"
            :places="props.horse.current_year_record_places ?? 0" :shows="props.horse.current_year_record_shows ?? 0"
            :earnings="props.horse.current_year_record_earnings ?? 0"
            :speed-rating="props.horse.best_bris_speed_most_recent_year ?? 0" :pedigree="''" :has-border="false"
            class="this-year">
        </HorseRecord>
    </div>
</template>

<style lang="scss" scoped>
.container {
    flex: 0 0 auto;

    align-self: start;
    display: grid;
    grid-template-columns: min-content min-content;
    grid-template-rows: auto auto;
    gap: 0;
}

.life {
    grid-column: 1;
    grid-row: 1;
}

.fst {
    grid-column: 2;
    grid-row: 1;
}

.this-year {
    grid-column: 1;
    grid-row: 2;
}
</style>