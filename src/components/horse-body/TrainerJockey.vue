<script setup lang="ts">
import { Horse } from "../../models/racecard";

const { horse } = defineProps<{
    horse: Horse;
}>();

</script>

<template>
    <div class="container font-small">
        <div class="color-accent-purple">Trainer/Jockey:</div>
        <div class="meet color-accent-yellow">Meet: ({{ horse.trainer_jockey_combo_starts_meet }} {{(horse.trainer_jockey_combo_wins_meet && horse.trainer_jockey_combo_starts_meet) ? ((horse.trainer_jockey_combo_wins_meet / horse.trainer_jockey_combo_starts_meet) * 100).toFixed(0) : "0" }}% {{ horse.trainer_jockey_combo_roi_meet?.toFixed(2) }})</div>
        <div class="l365 color-accent-green">Last 365: ({{ horse.trainer_jockey_combo_starts }} {{(horse.trainer_jockey_combo_wins && horse.trainer_jockey_combo_starts) ? ((horse.trainer_jockey_combo_wins / horse.trainer_jockey_combo_starts) * 100).toFixed(0) : "0" }}% {{ horse.trainer_jockey_combo_roi?.toFixed(2) }})</div>

    </div>
</template>

<style lang="scss" scoped>
.container {
    display: flex;
    flex-direction: row;
    gap: 0.25rem;
    margin-top: 1rem;
}

.meet {
    margin-left: 1rem;
}

.l365 {
    margin-left: 2rem;
}
</style>
