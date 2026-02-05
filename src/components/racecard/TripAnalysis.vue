<script setup lang="ts">
import { computed } from "vue";
import { Race } from "../../models/racecard";
import Panel from "../ui/Panel.vue";

const props = withDefaults(defineProps<{
    race: Race;
    print: boolean;
}>(), {
    print: false,
});

class TripInfo {
    program_number?: string;
    horse_name?: string;
    score?: number;
    comment?: string;
    surface?: string;
    distance?: number;
    date?: string;
    track?: string;
    adjPoints?: number;
    scratched?: boolean;
}

const tripData = computed(() => {
    const trips: Array<TripInfo> = [];

    for (const horse of props.race.horses || []) {
        if (!horse.trip_handicapping_info) continue;
        const cols = horse.trip_handicapping_info.split(",");
        const tripInfo = new TripInfo();
        tripInfo.scratched = horse.scratched;
        tripInfo.program_number = horse.program_number;
        tripInfo.horse_name = horse.horse_name;
        tripInfo.score = Number(cols[0]);
        tripInfo.comment = cols[1];
        tripInfo.surface = cols[2];
        tripInfo.distance = Number(cols[3]);
        tripInfo.date = cols[4];
        tripInfo.track = cols[5];
        tripInfo.adjPoints = Number(cols[6]);
        trips.push(tripInfo);
    }

    return trips
        .sort((a, b) => {
            const scoreA = Number.isFinite(a.score) ? (a.score as number) : Number.NEGATIVE_INFINITY;
            const scoreB = Number.isFinite(b.score) ? (b.score as number) : Number.NEGATIVE_INFINITY;
            if (scoreA !== scoreB) {
                return scoreB - scoreA;
            }
            const adjA = Number.isFinite(a.adjPoints) ? (a.adjPoints as number) : Number.NEGATIVE_INFINITY;
            const adjB = Number.isFinite(b.adjPoints) ? (b.adjPoints as number) : Number.NEGATIVE_INFINITY;
            return adjB - adjA;
        })
        .map((item) => item);
});


</script>

<template>
    <Panel :print="props.print">
        <div class="contents">
            <div class="color-accent-yellow">Trip Handicapping Model</div>
            <div class="trip-info" v-for="(trip, idx) in tripData">
                <div>{{ trip.program_number }}</div>
                <div>{{ trip.horse_name }}</div>
                <div>{{ trip.score?.toFixed(2) }}</div>
                <div>{{ trip.comment }}</div>
                <div>{{ trip.surface }}</div>
                <div>{{ trip.distance }}</div>
                <div>{{ trip.date }}</div>
                <div>{{ trip.track }}</div>
                <div>{{ trip.adjPoints }}</div>
            </div>
      </div>
    </Panel>
</template>

<style scoped lang="scss">
.contents {
    display: flex;
    flex-direction: column;
}

.trip-info {
    display: flex;
    flex-direction: row;
    gap: 1rem;
    margin-top: 0.5rem;
}
</style>
