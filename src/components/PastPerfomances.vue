<script setup lang="ts">
import { computed } from 'vue';
import { type Horse } from "../models/racecard";
import Transformers, { PositionLengthsBehind } from '../utils/transformers';

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

const firstCallPositions = computed(() => {
    const pps = props.horse?.past_performances || [];
    return pps.map(pp => Transformers.getPositionAndLengthsBehindStrings(
        Transformers.parseNumberOrNull(pp.first_call_position),
        Transformers.parseNumberOrNull(pp.first_call_between_lengths)
    ));
});

const secondCallPositions = computed(() => {
    const pps = props.horse?.past_performances || [];
    return pps.map(pp => Transformers.getPositionAndLengthsBehindStrings(
        Transformers.parseNumberOrNull(pp.second_call_position),
        Transformers.parseNumberOrNull(pp.second_call_between_lengths)
    ));
});

const stretchCallPositions = computed(() => {
    const pps = props.horse?.past_performances || [];
    return pps.map(pp => Transformers.getPositionAndLengthsBehindStrings(
        Transformers.parseNumberOrNull(pp.stretch_call_position),
        Transformers.parseNumberOrNull(pp.stretch_call_between_lengths)
    ));
});

const finishPositions = computed(() => {
    const pps = props.horse?.past_performances || [];
    return pps.map(pp => Transformers.getPositionAndLengthsBehindStrings(
        Transformers.parseNumberOrNull(pp.finish_position),
        Transformers.parseNumberOrNull(pp.finish_between_lengths)
    ));
});

function positionClass(position: string) {
    switch (position) {
        case '1':
            return 'color-accent-red';
        case '2':
            return 'color-accent-green';
        case '3':
            return 'color-accent-yellow';
        default:
            return '';
    }
}
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
            <div class="right-align">E1</div>
            <div class="right-align">E2</div>
            <div class="right-align">LP</div>
            <div class="right-align">SPD</div>
            <div class="right-align">PP</div>
            <div class="right-align move-right-small">ST</div>
            <div class="right-align move-right">1C</div>
            <div></div>
            <div class="right-align move-right">2C</div>
            <div></div>
            <div class="right-align move-right">Str</div>
            <div></div>
            <div class="right-align move-right">FIN</div>
            <div></div>
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
                <div class="right-align">{{ e1_e2_lp[i]?.[0] }}</div>
                <div class="right-align">{{ e1_e2_lp[i]?.[1] }}</div>
                <div class="right-align">{{ e1_e2_lp[i]?.[2] }}</div>
                <div class="right-align">{{ props.horse.past_performances[i].bris_speed_rating }}</div>
                <div class="right-align">{{ props.horse.past_performances[i].post_position }}</div>
                <div class="right-align" :class="positionClass(props.horse.past_performances[i].start_call_position.toString())">{{ props.horse.past_performances[i].start_call_position }}</div>

                <div class="right-align" :class="positionClass(firstCallPositions[i].position)">{{ firstCallPositions[i]?.position }}</div>
                <div :class="positionClass(firstCallPositions[i].position)"><span class="use-superscript">{{ firstCallPositions[i]?.lengthsBehind }}</span>{{ firstCallPositions[i]?.fraction }}</div>

                <div class="right-align" :class="positionClass(secondCallPositions[i].position)">{{ secondCallPositions[i]?.position }}</div>
                <div :class="positionClass(secondCallPositions[i].position)"><span class="use-superscript">{{ secondCallPositions[i]?.lengthsBehind }}</span>{{ secondCallPositions[i]?.fraction }}</div>

                <div class="right-align" :class="positionClass(stretchCallPositions[i].position)">{{ stretchCallPositions[i]?.position }}</div>
                <div :class="positionClass(stretchCallPositions[i].position)"><span class="use-superscript">{{ stretchCallPositions[i]?.lengthsBehind }}</span>{{ stretchCallPositions[i]?.fraction }}</div>

                <div class="right-align" :class="positionClass(finishPositions[i].position)">{{ finishPositions[i]?.position }}</div>
                <div :class="positionClass(finishPositions[i].position)"><span class="use-superscript">{{ finishPositions[i]?.lengthsBehind }}</span>{{ finishPositions[i]?.fraction }}</div>

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
        6rem 
        // 9. E1
        3rem 
        // 10. E2
        3rem 
        // 11. LP
        3rem
        // 12. SPD
        3rem
        // 13. PP
        3rem
        // 14. ST
        3rem
        // 15. 1C Position
        3rem
        // 16. 1C Lengths
        1rem
        // 17. 2C Position
        3rem
        // 18. 2C Lengths
        1rem
        // 19. Stretch Position
        3rem
        // 20. Stretch Lengths
        1rem
        // 19. Finish Position
        3rem
        // 20. Finish Lengths
        1rem;
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
    font-size: 0.8em;
    line-height: 0;
}

.left-padding {
    padding-left: 1rem;
}

.right-align {
    text-align: right;
}

.move-right {
    transform: translateX(0.4rem);
}
.move-right-small {
    transform: translateX(0.2rem);
}
</style>