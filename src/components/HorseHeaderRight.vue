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
            :speed-rating="props.horse.best_bris_speed_life ?? 0" :pedigree="''" :firstColumn="true" :has-border="true">
        </HorseRecord>
        <HorseRecord :label="'Fst'" :starts="props.horse.starts_fast_dirt ?? 0" :wins="props.horse.wins_fast_dirt ?? 0"
            :places="props.horse.places_fast_dirt ?? 0" :shows="props.horse.shows_fast_dirt ?? 0"
            :earnings="props.horse.earnings_fast_dirt ?? 0" :speed-rating="props.horse.best_bris_speed_fast_track ?? 0"
            :pedigree="props.horse.bris_dirt_pedigree_rating ?? ''" :firstColumn="false" :has-border="false">
        </HorseRecord>
        <HorseRecord :label="props.horse.current_year_record_year?.toString() ?? ''"
            :starts="props.horse.current_year_record_starts ?? 0" :wins="props.horse.current_year_record_wins ?? 0"
            :places="props.horse.current_year_record_places ?? 0" :shows="props.horse.current_year_record_shows ?? 0"
            :earnings="props.horse.current_year_record_earnings ?? 0"
            :speed-rating="props.horse.best_bris_speed_most_recent_year ?? 0" :pedigree="''" :firstColumn="true" :has-border="false">
        </HorseRecord>
        <HorseRecord :label="'Off:'"
            :starts="props.horse.lifetime_record_wet_starts ?? 0" :wins="props.horse.lifetime_record_wet_wins ?? 0"
            :places="props.horse.lifetime_record_wet_places ?? 0" :shows="props.horse.lifetime_record_wet_shows ?? 0"
            :earnings="props.horse.lifetime_record_wet_earnings ?? 0"
            :speed-rating="props.horse.best_bris_speed_off_track ?? 0" :pedigree="props.horse.bris_mud_pedigree_rating ?? ''" :firstColumn="false" :has-border="false">
        </HorseRecord>
        <HorseRecord :label="props.horse.previous_year_record_year?.toString() ?? ''"
            :starts="props.horse.previous_year_record_starts ?? 0" :wins="props.horse.previous_year_record_wins ?? 0"
            :places="props.horse.previous_year_record_places ?? 0" :shows="props.horse.previous_year_record_shows ?? 0"
            :earnings="props.horse.previous_year_record_earnings ?? 0"
            :speed-rating="props.horse.best_bris_speed_2nd_most_recent_year ?? 0" :pedigree="''" :firstColumn="true" :has-border="false">
        </HorseRecord>
        <HorseRecord :label="'Dis:'"
            :starts="props.horse.lifetime_record_todays_distance_starts ?? 0" :wins="props.horse.lifetime_record_todays_distance_wins ?? 0"
            :places="props.horse.lifetime_record_todays_distance_places ?? 0" :shows="props.horse.lifetime_record_todays_distance_shows ?? 0"
            :earnings="props.horse.lifetime_record_todays_distance_earnings ?? 0"
            :speed-rating="props.horse.best_bris_speed_distance ?? 0" :pedigree="props.horse.bris_dirt_pedigree_rating ?? ''" :firstColumn="false" :has-border="false">
        </HorseRecord>
        <HorseRecord :label="'Track:'"
            :starts="props.horse.lifetime_record_todays_track_starts ?? 0" :wins="props.horse.lifetime_record_todays_track_wins ?? 0"
            :places="props.horse.lifetime_record_todays_track_places ?? 0" :shows="props.horse.lifetime_record_todays_track_shows ?? 0"
            :earnings="props.horse.lifetime_record_todays_track_earnings ?? 0"
            :speed-rating="props.horse.best_bris_speed_todays_track ?? 0" :pedigree="''" :firstColumn="true" :has-border="false">
        </HorseRecord>
        <HorseRecord :label="'Trf:'"
            :starts="props.horse.lifetime_record_turf_starts ?? 0" :wins="props.horse.lifetime_record_turf_wins ?? 0"
            :places="props.horse.lifetime_record_turf_places ?? 0" :shows="props.horse.lifetime_record_turf_shows ?? 0"
            :earnings="props.horse.lifetime_record_turf_earnings ?? 0"
            :speed-rating="props.horse.best_bris_speed_turf ?? 0" :pedigree="props.horse.bris_turf_pedigree_rating ?? ''" :firstColumn="false" :has-border="false">
        </HorseRecord>
        <div></div>
        <HorseRecord :label="'AW:'"
            :starts="props.horse.lifetime_all_weather_starts ?? 0" :wins="props.horse.lifetime_all_weather_wins ?? 0"
            :places="props.horse.lifetime_all_weather_places ?? 0" :shows="props.horse.lifetime_all_weather_shows ?? 0"
            :earnings="props.horse.lifetime_all_weather_earnings ?? 0"
            :speed-rating="props.horse.best_bris_speed_all_weather_surface ?? 0" :pedigree="''" :firstColumn="false" :has-border="false">
        </HorseRecord>
    </div>
</template>

<style lang="scss" scoped>
.container {
    font-size: 1.3rem;
    flex: 0 0 25%;

    align-self: start;
    display: grid;
    grid-template-columns: min-content min-content;
    grid-template-rows: auto auto;
    gap: 0;
    margin-top: -0.3rem;
}
</style>