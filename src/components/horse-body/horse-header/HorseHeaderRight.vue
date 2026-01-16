<script setup lang="ts">
import type { Horse } from "../../../models/racecard";
import HorseRecord from "./HorseRecord.vue";

const props = defineProps<{
    horse: Horse;
}>();
</script>

<template>
    <div class="container">
        <HorseRecord :label="'Life:'" :starts="props.horse.lifetimeRecordStarts ?? 0"
            :wins="props.horse.lifetimeRecordWins ?? 0" :places="props.horse.lifetimeRecordPlaces ?? 0"
            :shows="props.horse.lifetimeRecordShows ?? 0" :earnings="props.horse.lifetimeRecordEarnings ?? 0"
            :speed-rating="props.horse.bestBrisSpeedLife ?? 0" :pedigree="''" :firstColumn="true" :has-border="true">
        </HorseRecord>
        <HorseRecord :label="'Fst'" :starts="props.horse.startsFastDirt ?? 0" :wins="props.horse.winsFastDirt ?? 0"
            :places="props.horse.placesFastDirt ?? 0" :shows="props.horse.showsFastDirt ?? 0"
            :earnings="props.horse.earningsFastDirt ?? 0" :speed-rating="props.horse.bestBrisSpeedFastTrack ?? 0"
            :pedigree="props.horse.brisDirtPedigreeRating ?? ''" :firstColumn="false" :has-border="false">
        </HorseRecord>
        <HorseRecord :label="props.horse.currentYearRecordYear?.toString() ?? ''"
            :starts="props.horse.currentYearRecordStarts ?? 0" :wins="props.horse.currentYearRecordWins ?? 0"
            :places="props.horse.currentYearRecordPlaces ?? 0" :shows="props.horse.currentYearRecordShows ?? 0"
            :earnings="props.horse.currentYearRecordEarnings ?? 0"
            :speed-rating="props.horse.bestBrisSpeedMostRecentYear ?? 0" :pedigree="''" :firstColumn="true" :has-border="false">
        </HorseRecord>
        <HorseRecord :label="'Off:'"
            :starts="props.horse.lifetimeRecordWetStarts ?? 0" :wins="props.horse.lifetimeRecordWetWins ?? 0"
            :places="props.horse.lifetimeRecordWetPlaces ?? 0" :shows="props.horse.lifetimeRecordWetShows ?? 0"
            :earnings="props.horse.lifetimeRecordWetEarnings ?? 0"
            :speed-rating="props.horse.bestBrisSpeedOffTrack ?? 0" :pedigree="props.horse.brisMudPedigreeRating ?? ''" :firstColumn="false" :has-border="false">
        </HorseRecord>
        <HorseRecord :label="props.horse.previousYearRecordYear?.toString() ?? ''"
            :starts="props.horse.previousYearRecordStarts ?? 0" :wins="props.horse.previousYearRecordWins ?? 0"
            :places="props.horse.previousYearRecordPlaces ?? 0" :shows="props.horse.previousYearRecordShows ?? 0"
            :earnings="props.horse.previousYearRecordEarnings ?? 0"
            :speed-rating="props.horse.bestBrisSpeed2ndMostRecentYear ?? 0" :pedigree="''" :firstColumn="true" :has-border="false">
        </HorseRecord>
        <HorseRecord :label="'Dis:'"
            :starts="props.horse.lifetimeRecordTodaysDistanceStarts ?? 0" :wins="props.horse.lifetimeRecordTodaysDistanceWins ?? 0"
            :places="props.horse.lifetimeRecordTodaysDistancePlaces ?? 0" :shows="props.horse.lifetimeRecordTodaysDistanceShows ?? 0"
            :earnings="props.horse.lifetimeRecordTodaysDistanceEarnings ?? 0"
            :speed-rating="props.horse.bestBrisSpeedDistance ?? 0" :pedigree="props.horse.brisDirtPedigreeRating ?? ''" :firstColumn="false" :has-border="false">
        </HorseRecord>
        <HorseRecord :label="'Track:'"
            :starts="props.horse.lifetimeRecordTodaysTrackStarts ?? 0" :wins="props.horse.lifetimeRecordTodaysTrackWins ?? 0"
            :places="props.horse.lifetimeRecordTodaysTrackPlaces ?? 0" :shows="props.horse.lifetimeRecordTodaysTrackShows ?? 0"
            :earnings="props.horse.lifetimeRecordTodaysTrackEarnings ?? 0"
            :speed-rating="props.horse.bestBrisSpeedTodaysTrack ?? 0" :pedigree="''" :firstColumn="true" :has-border="false">
        </HorseRecord>
        <HorseRecord :label="'Trf:'"
            :starts="props.horse.lifetimeRecordTurfStarts ?? 0" :wins="props.horse.lifetimeRecordTurfWins ?? 0"
            :places="props.horse.lifetimeRecordTurfPlaces ?? 0" :shows="props.horse.lifetimeRecordTurfShows ?? 0"
            :earnings="props.horse.lifetimeRecordTurfEarnings ?? 0"
            :speed-rating="props.horse.bestBrisSpeedTurf ?? 0" :pedigree="props.horse.brisTurfPedigreeRating ?? ''" :firstColumn="false" :has-border="false">
        </HorseRecord>
        <div></div>
        <HorseRecord :label="'AW:'"
            :starts="props.horse.lifetimeAllWeatherStarts ?? 0" :wins="props.horse.lifetimeAllWeatherWins ?? 0"
            :places="props.horse.lifetimeAllWeatherPlaces ?? 0" :shows="props.horse.lifetimeAllWeatherShows ?? 0"
            :earnings="props.horse.lifetimeAllWeatherEarnings ?? 0"
            :speed-rating="props.horse.bestBrisSpeedAllWeatherSurface ?? 0" :pedigree="''" :firstColumn="false" :has-border="false">
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