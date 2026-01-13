<script setup lang="ts">
import { computed } from 'vue';
import { type Horse } from "../models/racecard";
import Transformers from '../utils/transformers';

const props = defineProps<{
    horse: Horse;
}>();

const fraction_1 = computed(() => {
    const pps = props.horse?.past_performances || [];
    return pps.map(pp => Transformers.getFractionalTimeString(pp?.fraction_1 ?? null));
});

const fraction_2 = computed(() => {
    const pps = props.horse?.past_performances || [];
    return pps.map(pp => Transformers.getFractionalTimeString(pp?.fraction_2 ?? null));
});

const fraction_3 = computed(() => {
    const pps = props.horse?.past_performances || [];
    return pps.map(pp => Transformers.getFractionalTimeString(pp?.fraction_3 ?? null));
});

const final_time = computed(() => {
    const pps = props.horse?.past_performances || [];
    return pps.map(pp => Transformers.getFractionalTimeString(pp?.final_time ?? null));
});

const race_classification = computed(() => {
    const pps = props.horse?.past_performances || [];
    return pps.map(pp => Transformers.getPPRaceClassification(pp));
});

const e1_e2_lp = computed(() => {
    const pps = props.horse?.past_performances || [];
    return pps.map(pp => Transformers.getE1E2AndLatePaceString(pp));
});

</script>

<template>
    <div>
        <div v-if="props.horse.past_performances && props.horse.past_performances[0] && props.horse.past_performances[0].race_date !== ''"
            class="pp-grid title-row color-accent-green">
            <div>DATE TRK</div>
            <div></div>
            <div>DIST</div>
            <div></div>
            <div></div>
            <div></div>
            <div></div>
            <div>RACETYPE</div>
            <div>E1</div>
            <div>E2</div>
            <div>LP</div>
        </div>

        <template v-for="(_, i) in Array(10)" :key="i">
            <div class="claimed-row"
                v-if="props.horse.past_performances?.[i]?.alternate_comment_line?.startsWith('Claimed')">
                <div class="claimed-cell">
                    <div class="color-accent-purple" :title="props.horse.past_performances[i].alternate_comment_line">
                        {{ props.horse.past_performances[i].alternate_comment_line }} ( as of {{
                            props.horse.past_performances[i].claimed_and_trainer_switches_1 }} ): ( {{
                            props.horse.past_performances[i].claimed_and_trainer_switches_2 }} {{
                            props.horse.past_performances[i].claimed_and_trainer_switches_3 }}-{{
                            props.horse.past_performances[i].claimed_and_trainer_switches_4 }}-{{
                            props.horse.past_performances[i].claimed_and_trainer_switches_5 }}
                        {{
                            Transformers.createPercentageString(Transformers.parseNumberOrNull(props.horse.past_performances[i].claimed_and_trainer_switches_3),
                                Transformers.parseNumberOrNull(props.horse.past_performances[i].claimed_and_trainer_switches_2))
                        }} )
                    </div>
                </div>
            </div>

            <div class="pp-grid perf-row"
                v-if="props.horse.past_performances && props.horse.past_performances[i] && props.horse.past_performances[i].race_date !== ''">
                <div :style="{
                    borderBottom: (i === 0 && props.horse.past_performances?.[i]?.days_since_last_race && props.horse.past_performances[i].days_since_last_race > 45)
                        ? '1px solid var(--accent-red)'
                        : undefined
                }">
                    {{ Transformers.formatDateShort(props.horse.past_performances[i].race_date) }}{{
                        props.horse.past_performances[i].track_code }}<span class="use-superscript">{{
                        props.horse.past_performances[i].race_number }}</span>
                </div>
                <div class="color-accent-yellow surface-indicator">{{
                    Transformers.getSurfaceString(props.horse.past_performances[i]) }}</div>
                <div>{{ Transformers.getShortLength(props.horse.past_performances[i].distance) }} <span>{{
                    props.horse.past_performances[i].track_condition.toLowerCase() }}</span><span
                        v-if="props.horse.past_performances[i].sealed_track_indicator" class="use-superscript">s</span>
                </div>
                <div class="left-padding">{{ fraction_1[i]?.[0] }}<span class="use-superscript">{{ fraction_1[i]?.[1]
                        }}</span></div>
                <div>{{ fraction_2[i]?.[0] }}<span class="use-superscript">{{ fraction_2[i]?.[1] }}</span></div>
                <div>{{ fraction_3[i]?.[0] }}<span class="use-superscript">{{ fraction_3[i]?.[1] }}</span></div>
                <div>{{ final_time[i]?.[0] }}<span class="use-superscript">{{ final_time[i]?.[1] }}</span></div>
                <div><span class="color-accent-yellow">{{ race_classification[i]?.[0] }}</span>{{
                    race_classification[i]?.[1] }}</div>
                <div>{{ e1_e2_lp[i]?.[0] }}</div>
                <div>{{ e1_e2_lp[i]?.[1] }}</div>
                <div>{{ e1_e2_lp[i]?.[2] }}</div>
            </div>
        </template>
    </div>
</template>

<style lang="scss" scoped>
.pp-grid {
    font-size: 1.4rem;
    margin-top: 0.25rem;
    display: grid;
    grid-template-columns:
        // 1. DATETRK
        9rem 
        // 2. Surface Prefix — narrow minimum so gap is small
        1rem
        // 3. DIST 
        3rem 
        // 4. Fraction # 1
        4rem 
        // 5. Fraction # 2
        3rem 
        // 6. Fraction # 3
        3rem 
        // 7. Final Time
        3rem 
        // 8. RACETYPE
        8rem 
        // 9. E1
        3rem 
        // 10. E2
        3rem 
        // 11. LP
        3rem;
}

.claimed-row {
    display: block;
    margin: 0.12rem 8rem;
}

.claimed-cell {
    min-height: 1.6rem;
    min-width: 0;
    overflow: hidden;
}

.claimed-cell>.color-accent-purple {
    display: block;
    max-height: 1.6rem;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-size: 0.95em;
}

.surface-indicator {
    padding-right: 0.25rem;
    text-align: right;
}

.use-superscript {
    vertical-align: super;
    font-size: 0.7em;
    line-height: 0;
}

.left-padding {
    padding-left: 1rem;
}
</style>