<script setup lang="ts">
import type { Workout } from "../models/racecard";
import Transformers from "../utils/transformers";

const props = defineProps<{
    workouts: Workout[];
}>();

class WorkoutTime {
    constructor(public time: string, public fraction: string) {}
}

const bullets = props.workouts.map(w => w.rank === 1 ? "●" : "");
const surface = props.workouts.map(w => w.main_inner_track_indicator === "MT" ? " " : w.main_inner_track_indicator === "TT" ? "tr.t" : "***" +w.main_inner_track_indicator);

const workoutTimes = props.workouts.map(w => {
    let time = Transformers.getFractionalTimeString(w.time);
    return new WorkoutTime(time[0], time[1]);
});

</script>

<template>
    <div v-if="props.workouts.length > 0" class="workouts">
        <div v-for="(w, i) in props.workouts" :key="i" class="workout">
            <div class="bullet color-accent-red">{{ bullets[i] }}</div>
            <div>{{ Transformers.formatDateShort(w.date) }}</div>
            <div>{{ surface[i] }}</div>
            <div>{{ Transformers.getShortLength(w.distance) }}</div>
            <div>{{ w.condition }}</div>
            <div class="align-right">{{ workoutTimes[i].time }}</div>
            <div class="align-left use-superscript">{{ workoutTimes[i].fraction }}</div>
            <div>8</div>
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
    grid-template-columns: 2rem 5rem 2rem 2rem 2rem 2rem 2rem 4rem;
}

.bullet {
    transform: translateY(-0.4rem);
}

</style>
