<script setup lang="ts">
import { type Horse } from "../../models/racecard";
import Transformers from "../../utils/transformers";
import Finishers from './Finishers.vue';
import Tooltip from '../ui/Tooltip.vue';

const props = withDefaults(defineProps<{
    horse: Horse, print: boolean;
}>(), {
    print: false,
});

const pps = props.horse?.pastPerformances ?? [];

const fraction1 = pps.map(pp => Transformers.getFractionalTimeString(pp?.fraction1 ?? null));
const fraction2 = pps.map(pp => Transformers.getFractionalTimeString(pp?.fraction2 ?? null));
const fraction3 = pps.map(pp => Transformers.getFractionalTimeString(pp?.fraction3 ?? null));
const finalTime = pps.map(pp => Transformers.getFractionalTimeString(pp?.finalTime ?? null));
const race_classification = pps.map(pp => Transformers.getPPRaceClassification(pp));
const e1_e2_lp = pps.map(pp => Transformers.getE1E2AndLatePaceString(pp));

const firstCallPositions = pps.map(pp => Transformers.getPositionAndLengthsBehindStrings(
    Transformers.parseNumberOrNull(pp.firstCallPosition),
    Transformers.parseNumberOrNull(pp.firstCallBetweenLengths)
));
const secondCallPositions = pps.map(pp => Transformers.getPositionAndLengthsBehindStrings(
    Transformers.parseNumberOrNull(pp.secondCallPosition),
    Transformers.parseNumberOrNull(pp.secondCallBetweenLengths)
));
const stretchCallPositions = pps.map(pp => Transformers.getPositionAndLengthsBehindStrings(
    Transformers.parseNumberOrNull(pp.stretchCallPosition),
    Transformers.parseNumberOrNull(pp.stretchCallBetweenLengths)
));
const finishPositions = pps.map(pp => Transformers.getPositionAndLengthsBehindStrings(
    Transformers.parseNumberOrNull(pp.finishPosition),
    Transformers.parseNumberOrNull(pp.finishBetweenLengths)
));

const cols = !props.print ?
        '9rem 1rem 4rem 4rem 3rem 3rem 3rem 8rem 3rem 3rem 3rem 3rem 3rem 3rem 3rem 1rem 3rem 1rem 3rem 1rem 3rem 3rem 10rem 1rem 4rem 30rem 22rem 2rem' :
        '7rem 1rem 3rem 3rem 2rem 2rem 3rem 6rem 2rem 2rem 2rem 2rem 2rem 2rem 2rem 1rem 2rem 1rem 2rem 1rem 2rem 2rem 8rem 1rem 3rem 25rem 16rem 1rem';

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
    <div class="font-small">
        <div v-if="pps && pps[0] && pps[0].raceDate !== ''"
            class="pp-grid title-row color-accent-green" :style="{ gridTemplateColumns: cols }">
            <div>DATE TRK</div>
            <div></div>
            <div>DIST</div>
            <div></div>
            <div></div>
            <div></div>
            <div></div>
            <div>RACETYPE</div>
            <div class="align-right">E1</div>
            <div class="align-right">E2</div>
            <div class="align-right">LP</div>
            <div class="align-right">SPD</div>
            <div class="align-right">PP</div>
            <div class="align-right move-right-small">ST</div>
            <div class="align-right move-right">1C</div>
            <div></div>
            <div class="align-right move-right">2C</div>
            <div></div>
            <div class="align-right move-right">Str</div>
            <div></div>
            <div class="align-right move-right">FIN</div>
            <div></div>
            <div>JOCKEY</div>
            <div></div>
            <div class="align-right">ODDS</div>
            <div class="move-right-large">Top Finishers</div>
            <div class="align-right">Comments</div>
            <div></div>
        </div>

        <template v-for="(_, i) in Array(10)" :key="i">
            <div class="claimed-row"
                v-if="pps?.[i]?.alternateCommentLine?.startsWith('Claimed')">
                <div class="claimed-cell">
                    <div class="color-accent-purple" :title="pps[i].alternateCommentLine">
                        {{ pps[i].alternateCommentLine }} ( as of {{
                            pps[i].claimedAndTrainerSwitches1 }} ): ( {{
                            pps[i].claimedAndTrainerSwitches2 }} {{
                            pps[i].claimedAndTrainerSwitches3 }}-{{
                            pps[i].claimedAndTrainerSwitches4 }}-{{
                            pps[i].claimedAndTrainerSwitches5 }}
                        {{
                            Transformers.createPercentageString(Transformers.parseNumberOrNull(pps[i].claimedAndTrainerSwitches3),
                                Transformers.parseNumberOrNull(pps[i].claimedAndTrainerSwitches2))
                        }} )
                    </div>
                </div>
            </div>

            <div class="pp-grid perf-row" :style="{ gridTemplateColumns: cols }"
                v-if="pps && pps[i] && pps[i].raceDate !== ''">
                <div :style="{
                    borderBottom: (i === 0 && pps?.[i]?.daysSinceLastRace && pps[i].daysSinceLastRace > 45)
                        ? '1px solid var(--accent-red)'
                        : undefined
                }">
                    {{ Transformers.formatDateShort(pps[i].raceDate) }}{{
                        pps[i].trackCode }}<span class="use-superscript">{{
                        pps[i].raceNumber }}</span>
                </div>
                <div class="color-accent-yellow surface-indicator">{{
                    Transformers.getSurfaceString(pps[i]) }}</div>
                <div>{{ Transformers.getShortLength(pps[i].distance) }} <span>{{
                    pps[i].trackCondition.toLowerCase() }}</span><span
                        v-if="pps[i].sealedTrackIndicator" class="use-superscript">s</span>
                </div>
                <div class="left-padding">{{ fraction1[i]?.int }}<span class="use-superscript">{{ fraction1[i]?.fraction
                        }}</span></div>
                <div>{{ fraction2[i]?.int }}<span class="use-superscript">{{ fraction2[i]?.fraction }}</span></div>
                <div>{{ fraction3[i]?.int }}<span class="use-superscript">{{ fraction3[i]?.fraction }}</span></div>
                <div>{{ finalTime[i]?.int }}<span class="use-superscript">{{ finalTime[i]?.fraction }}</span></div>
                <div><span class="color-accent-yellow">{{ race_classification[i]?.prefix }}</span>{{
                    race_classification[i]?.classification }}</div>
                <div class="align-right">{{ e1_e2_lp[i]?.e1 }}</div>
                <div class="align-right">{{ e1_e2_lp[i]?.e2 }}</div>
                <div class="align-right">{{ e1_e2_lp[i]?.latePace }}</div>
                <div class="align-right">{{ pps[i].brisSpeedRating }}</div>
                <div class="align-right">{{ pps[i].postPosition }}</div>
                <div class="align-right" :class="positionClass(pps[i].startCallPosition.toString())">{{ pps[i].startCallPosition }}</div>

                <div class="align-right" :class="positionClass(firstCallPositions[i].position)">{{ firstCallPositions[i]?.position }}</div>
                <div :class="positionClass(firstCallPositions[i].position)"><span class="use-superscript">{{ firstCallPositions[i]?.lengthsBehind }}</span>{{ firstCallPositions[i]?.fraction }}</div>

                <div class="align-right" :class="positionClass(secondCallPositions[i].position)">{{ secondCallPositions[i]?.position }}</div>
                <div :class="positionClass(secondCallPositions[i].position)"><span class="use-superscript">{{ secondCallPositions[i]?.lengthsBehind }}</span>{{ secondCallPositions[i]?.fraction }}</div>

                <div class="align-right" :class="positionClass(stretchCallPositions[i].position)">{{ stretchCallPositions[i]?.position }}</div>
                <div :class="positionClass(stretchCallPositions[i].position)"><span class="use-superscript">{{ stretchCallPositions[i]?.lengthsBehind }}</span>{{ stretchCallPositions[i]?.fraction }}</div>

                <div class="align-right" :class="positionClass(finishPositions[i].position)">{{ finishPositions[i]?.position }}</div>
                <div :class="positionClass(finishPositions[i].position)"><span class="use-superscript">{{ finishPositions[i]?.lengthsBehind }}</span>{{ finishPositions[i]?.fraction }}<span v-if="pps[i].finishPosition !== pps[i].moneyPosition">*</span></div>

                <div>{{ Transformers.getJockeyName(pps[i].jockey) }}<span class="use-superscript">{{ pps[i].weight }}</span></div>

                <div><span v-if="pps[i].equipment === 'b'">b</span><span v-if="pps[i].frontBandagesIndicator === '1'">f</span></div>

                <div class="align-right">{{pps[i].favoriteIndicator === "1" ? "*" : "" }}{{ pps[i].odds?.toFixed(2) }}</div>

                <Finishers :pp="pps[i]" class="move-right-large"/>

                <div class="comment">
                    <Tooltip :text="Transformers.stripLeadingDate(pps[i].extendedStartComment)">
                         {{ pps[i].tripComment }}
                    </Tooltip>
                </div>

                <div class="align-right">{{ pps[i].entrants }}</div>
            </div>
        </template>
    </div>
</template>

<style lang="scss" scoped>
.pp-grid {
    margin-top: 0.25rem;
    display: grid;
    // grid-template-columns:
    //     // 1. DATETRK
    //     9rem 
    //     // 2. Surface Prefix — narrow minimum so gap is small
    //     1rem
    //     // 3. DIST 
    //     4rem 
    //     // 4. Fraction # 1
    //     4rem 
    //     // 5. Fraction # 2
    //     3rem 
    //     // 6. Fraction # 3
    //     3rem 
    //     // 7. Final Time
    //     3rem 
    //     // 8. RACETYPE
    //     8rem 
    //     // 9. E1
    //     3rem 
    //     // 10. E2
    //     3rem 
    //     // 11. LP
    //     3rem
    //     // 12. SPD
    //     3rem
    //     // 13. PP
    //     3rem
    //     // 14. ST
    //     3rem
    //     // 15. 1C Position
    //     3rem
    //     // 16. 1C Lengths
    //     1rem
    //     // 17. 2C Position
    //     3rem
    //     // 18. 2C Lengths
    //     1rem
    //     // 19. Stretch Position
    //     3rem
    //     // 20. Stretch Lengths
    //     1rem
    //     // 19. Finish Position
    //     3rem
    //     // 20. Finish Lengths
    //     3rem
    //     // 21. Jockey
    //     10rem
    //     // 22. Bandages/Favorite indicator
    //     1rem
    //     // 23. Odds
    //     4rem
    //     // 24. Top Finishers
    //     30rem
    //     // 25. Comments
    //     22rem
    //     // 26. Entries
    //     2rem;
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

.move-right {
    transform: translateX(0.4rem);
}

.move-right-small {
    transform: translateX(0.2rem);
}

.move-right-large {
    transform: translateX(0.8rem);
}

.comment {
    z-index: 1;
    justify-self: end;
    text-align: right;
    align-self: start;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    align-items: flex-end;
}
</style>
