<script setup lang="ts">
import type { Workout } from "../../models/racecard";
import Transformers from "../../utils/transformers";

const props = defineProps<{
    workouts: Workout[];
}>();

const visibleWorkouts = props.workouts.filter(w => w.date !== null && w.date !== '');

const bullets = props.workouts.map(w => w.rank === 1 ? "●" : "");
const surface = props.workouts.map(w => w.mainInnerTrackIndicator === "MT" ? " " : w.mainInnerTrackIndicator === "TT" ? "tr.t" : "***" + w.mainInnerTrackIndicator);

const workoutTimes = props.workouts.map(w => {
    let time = Transformers.getFractionalTimeString(w.time);
    return { time: time.int, fraction: time.fraction };
});

</script>

<template>
    <div v-if="visibleWorkouts.length > 0" class="workouts">
        <div v-for="(w, i) in visibleWorkouts" :key="i" class="workout">
            <div class="bullet color-accent-red align-right">{{ bullets[i] }}</div>
            <div>{{ Transformers.formatDateShort(w.date) }}</div>
            <div>{{ surface[i] }}</div>
            <div>{{ Transformers.getShortLength(w.distance) }}</div>
            <div>{{ w.condition }}</div>
            <div class="align-right">{{ workoutTimes[i].time }}</div>
            <div class="align-left use-superscript">{{ workoutTimes[i].fraction }}</div>
            <div>{{ w.description }}</div>
            <div class="align-right">{{ w.rank }}/{{ w.workoutsThatDayDistance }}</div>
        </div>
    </div>
</template>

<style lang="scss" scoped>
.workouts {
    font-size: 1.4rem;
    display: flex;
    flex-direction: row;
    gap: 0.5rem;
    margin-top: 1rem;
}

.workout {
    display: grid;
    grid-template-columns: 2rem 5rem 2rem 2rem 2rem 2rem 2rem 1rem 5rem;
}

.workout:nth-child(odd) {
    color: var(--accent-yellow);
}

.workout:nth-child(even) {
    color: var(--accent-green);
}

.bullet {
    transform: translateY(-0.4rem);
}
</style>
