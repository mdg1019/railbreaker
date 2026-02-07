<script setup lang="ts">
import { computed } from "vue";
import { Race } from "../../models/racecard";
import Panel from "../ui/Panel.vue";
import Transformers from "../../utils/transformers";
import { useRacecardStateStore } from "../../stores/racecardStateStore";

const props = withDefaults(defineProps<{
    race: Race;
    print: boolean;
}>(), {
    print: false,
});

const racecardStateStore = useRacecardStateStore();

const tripData = computed(() => {
    return racecardStateStore.tripData;
});

const tripColumns = computed(() => {
    const trips = tripData.value;
    if (trips.length <= 6) return [trips];
    const mid = Math.ceil(trips.length / 2);
    return [trips.slice(0, mid), trips.slice(mid)];
});

const formatComment = (comment?: string) => {
    if (!comment) return comment;
    if (/^\d+w$/i.test(comment)) return comment;
    return Transformers.capitalize(comment);
};

</script>

<template>
    <Panel :print="props.print">
        <div class="contents">
            <div class="trip-title color-accent-yellow">Trip Handicapping Model</div>
            <div class="trip-info-columns">
                <div class="trip-info" :class="{ 'is-print': props.print, 'font-extra-large': props.print }" v-for="(column, colIdx) in tripColumns" :key="colIdx">
                    <div class="trip-info-header">
                        <div class="color-accent-yellow">#</div>
                        <div class="color-accent-yellow">Horse</div>
                        <div class="color-accent-yellow">Score</div>
                        <div class="color-accent-yellow">Comment</div>
                        <div class="color-accent-yellow">Surf</div>
                        <div class="color-accent-yellow">Dist</div>
                        <div class="color-accent-yellow">Date</div>
                        <div class="color-accent-yellow">Track</div>
                        <div class="color-accent-yellow">Adj</div>
                    </div>
                    <div
                        class="trip-info-row"
                        :class="{ 'is-scratched': trip.scratched }"
                        v-for="(trip, idx) in column"
                        :key="idx"
                    >
                        <div>{{ trip.program_number }}</div>
                        <div>{{ Transformers.capitalize(trip.horse_name!) }}</div>
                        <div>{{ trip.score?.toFixed(2) }}</div>
                        <div>{{ formatComment(trip.comment) }}</div>
                        <div>{{ trip.surface }}</div>
                        <div>{{ trip.distance?.toFixed(1) }}</div>
                        <div>{{ trip.date }}</div>
                        <div>{{ trip.track }}</div>
                        <div>{{ trip.adjPoints?.toFixed(2) }}</div>
                    </div>
                </div>
            </div>
        </div>
    </Panel>
</template>

<style scoped lang="scss">
.contents {
    display: flex;
    flex-direction: column;
}

.trip-info-columns {
    display: flex;
    gap: 2rem;
}

.trip-info {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    margin-top: 1rem;
    flex: 1 1 0;
    --trip-grid-columns: 2rem 18rem 5rem 8rem 5rem 2rem 10rem 5rem 5rem;
    --trip-grid-width: calc(2rem + 18rem + 5rem + 8rem + 5rem + 2rem + 10rem + 5rem + 5rem + 8rem);
}

.trip-info.is-print {
    --trip-grid-columns: 2rem 10rem 4rem 6rem 4rem 2rem 8rem 4rem 4rem;
    --trip-grid-width: calc(2rem + 10rem + 4rem + 6rem + 4rem + 2rem + 8rem + 4rem + 4rem + 8rem);
}

.trip-info-header,
.trip-info-row {
    display: grid;
    grid-template-columns: var(--trip-grid-columns);
    column-gap: 1rem;
    align-items: baseline;
    position: relative;
}

.trip-info-row.is-scratched::after {
    content: "";
    position: absolute;
    left: 0;
    width: var(--trip-grid-width);
    top: 50%;
    height: 2px;
    background: var(--accent-red);
    z-index: 2;
    pointer-events: none;
}

.trip-info :is(.trip-info-header, .trip-info-row) > div:nth-child(1),
.trip-info :is(.trip-info-header, .trip-info-row) > div:nth-child(3),
.trip-info :is(.trip-info-header, .trip-info-row) > div:nth-child(4),
.trip-info :is(.trip-info-header, .trip-info-row) > div:nth-child(6),
.trip-info :is(.trip-info-header, .trip-info-row) > div:nth-child(9) {
    text-align: right;
}

.trip-info :is(.trip-info-header, .trip-info-row) > div:nth-child(2),
.trip-info :is(.trip-info-header, .trip-info-row) > div:nth-child(5),
.trip-info :is(.trip-info-header, .trip-info-row) > div:nth-child(7),
.trip-info :is(.trip-info-header, .trip-info-row) > div:nth-child(8) {
    padding-left: 2rem;
}

.trip-info.is-print :is(.trip-info-header, .trip-info-row) > div:nth-child(2),
.trip-info.is-print :is(.trip-info-header, .trip-info-row) > div:nth-child(5),
.trip-info.is-print :is(.trip-info-header, .trip-info-row) > div:nth-child(7),
.trip-info.is-print :is(.trip-info-header, .trip-info-row) > div:nth-child(8) {
    padding-left: 0;
}

.panel.print .trip-title {
    font-size: 1.2rem;
}
</style>
