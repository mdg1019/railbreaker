<script setup lang="ts">
import type { Horse } from "../models/racecard";
import Panel from "./Panel.vue";
import HorseHeaderLeft from "./HorseHeaderLeft.vue";
import HorseHeaderRight from "./HorseHeaderRight.vue";
import HorseHeaderCenter from "./HorseHeaderCenter.vue";

const props = defineProps<{ horse: Horse, primePowerComparisons: Array<[number | string, string, string]> }>();
</script>

<template>
    <Panel>
        <div class="horse">
            <div class="horse-header">

                <HorseHeaderLeft :horse="props.horse" />  

                <HorseHeaderCenter
                    :horse="props.horse"
                    :primePowerComparisons="props.primePowerComparisons" />

                <HorseHeaderRight :horse="props.horse" />
            </div>

            <div class="horse-body">
                <!-- additional horse info can go here -->
            </div>
        </div>
    </Panel>
</template>

<style scoped lang="scss">
.horse {
    font-family: "MGSansCondensed", sans-serif;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
}

.horse-header {
    display: flex;
    flex-direction: row;
    gap: 1rem;
    align-items: center;
}
</style>
