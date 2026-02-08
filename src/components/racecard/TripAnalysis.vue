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

const trips = computed(() => tripData.value);

function formatTripComment(comment?: string | null): string {
    if (!comment) return "";
    const withSpaces = comment.replace(/_/g, " ");
    return withSpaces.replace(/:\s*([a-z])/g, (_match, letter: string) => `: ${letter.toUpperCase()}`);
}
</script>

<template>
    <Panel :print="props.print">
        <div class="contents">
            <div class="trip-title color-accent-yellow">Trip Handicapping Model</div>
            <div class="color-accent-yellow">Looks back 3 races not over 60 days old.</div>
            <div class="color-accent-yellow">Scoring: <span class="color-accent-green">1st Trip</span> Good: 50, Bad: -50, <span class="color-accent-green">2nd Trip</span> Good: 30, Bad: -30, <span class="color-accent-green">3rd Trip</span> Good: 10, Bad: -10. <span class="color-accent-green">Excusables</span> always count as 0.</div>
            <div class="trip-info font-small" :class="{ 'is-print': props.print, 'font-extra-large': props.print }">
                <div class="trip-info-header">
                    <div class="color-accent-yellow">#</div>
                    <div class="color-accent-yellow">Horse</div>
                    <div class="color-accent-yellow">Score</div>
                    <div class="color-accent-yellow">Days Back</div>
                    <div class="color-accent-yellow">Trip # 1</div>
                    <div class="color-accent-yellow">Days Back</div>
                    <div class="color-accent-yellow">Trip # 2</div>
                    <div class="color-accent-yellow">Days Back</div>
                    <div class="color-accent-yellow">Trip # 3</div>
                </div>
                <div
                    class="trip-info-row"
                    :class="{ 'is-scratched': trip.scratched }"
                    v-for="(trip, idx) in trips"
                    :key="idx"
                >
                    <div>{{ trip.program_number }}</div>
                    <div>{{ Transformers.capitalize(trip.horse_name!) }}</div>
                    <div>{{ trip.score }}</div>
                    <div>{{ trip.days_back_1 }}</div>
                    <div>{{ formatTripComment(trip.comment_1) }}</div>
                    <div>{{ trip.days_back_2 ? trip.days_back_2 : "" }}</div>
                    <div>{{ trip.days_back_2 ? formatTripComment(trip.comment_2) : "" }}</div>
                    <div>{{ trip.days_back_3 ? trip.days_back_3 : "" }}</div>
                    <div>{{ trip.days_back_3 ? formatTripComment(trip.comment_3) : "" }}</div>
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

.trip-info {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    margin-top: 1rem;
    flex: 1 1 0;
    --trip-grid-columns: 1rem 10rem 3rem 5em 15rem 6rem 15rem 6rem 15rem;
    --trip-grid-width: calc(1rem + 10rem + 3rem + 5em + 15rem + 6rem + 15rem + 6rem + 15rem);
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

.trip-info :is(.trip-info-header, .trip-info-row) > div:nth-child(1) {
    text-align: right;
}
</style>
