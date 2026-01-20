<script setup lang="ts">
import { Horse } from "../../models/racecard";

const { horse } = defineProps<{
    horse: Horse;
}>();

</script>

<template>
    <div class="container font-small">
        <div class="color-accent-purple">Trainer/Jockey:</div>
        <div class="meet color-accent-yellow">Meet: ({{ horse.trainerJockeyComboStartsMeet }} {{(horse.trainerJockeyComboWinsMeet && horse.trainerJockeyComboStartsMeet) ? ((horse.trainerJockeyComboWinsMeet / horse.trainerJockeyComboStartsMeet) * 100).toFixed(0) : "0" }}% {{ horse.trainerJockeyComboRoiMeet?.toFixed(2) }})</div>
        <div class="l365 color-accent-green">Last 365: ({{ horse.trainerJockeyComboStarts }} {{(horse.trainerJockeyComboWins && horse.trainerJockeyComboStarts) ? ((horse.trainerJockeyComboWins / horse.trainerJockeyComboStarts) * 100).toFixed(0) : "0" }}% {{ horse.trainerJockeyComboRoi?.toFixed(2) }})</div>

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
