<script setup lang="ts">
import { onMounted, onBeforeUnmount, ref, nextTick, computed } from "vue";
import { useRouter } from "vue-router";
import { emit, listen, type UnlistenFn } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type { Racecard } from "../models/racecard";
import { RaceCardPrintPayload } from "../models/print";
import { PRINT_PAYLOAD_EVENT, PRINT_READY_EVENT } from "../utils/openPrintWindowEvent";
import RacecardHeader from "../components/racecard/RacecardHeader.vue";
import RaceDetails from "../components/racecard/RaceDetails.vue";
import Horse from "../components/racecard/Horse.vue";
import { computePrimePowerComparisons } from "../utils/computePrimePowerComparisons";
import TripAnalysis from "../components/racecard/TripAnalysis.vue";
import "../scss/_main.scss";

const payload = ref<RaceCardPrintPayload | null>(null);

let hasPrinted = false;
const router = useRouter();

const racecard = computed(() => {
    const value = payload.value;
    if (!value) {
        return null;
    }
    if (typeof value === "object" && "raceCard" in value) {
        return (value as RaceCardPrintPayload).raceCard ?? null;
    }
    return value as Racecard;
});

const primePowerComparisonsByRace = computed<Record<number, Array<[number | string, string, string]>>>(() => {
    const result: Record<number, Array<[number | string, string, string]>> = {};
    const rc = racecard.value;
    if (!rc) {
        return result;
    }
    const raceCount = rc.races?.length ?? 0;
    for (let raceIdx = 0; raceIdx < raceCount; raceIdx += 1) {
        const raceNo = raceIdx + 1;
        result[raceNo] = computePrimePowerComparisons(rc, raceNo);
    }
    return result;
});

async function handlePayload(value: Racecard | RaceCardPrintPayload) {
    const nextPayload = ("raceCard" in value)
        ? (value as RaceCardPrintPayload)
        : new RaceCardPrintPayload(value as Racecard, []);
    if (!nextPayload.printRaces?.length) {
        const raceCount = nextPayload.raceCard?.races?.length ?? 0;
        nextPayload.printRaces = Array.from({ length: raceCount }, (_, idx) => idx + 1);
    }
    payload.value = nextPayload;
    if (!hasPrinted) {
        hasPrinted = true;
        await doPrintAndClose();
    }
}

let unlisten: UnlistenFn | null = null

async function waitForFonts() {
    const anyDoc = document as any;
    if (anyDoc.fonts?.ready) {
        try { await anyDoc.fonts.ready; } catch { }
    }
}

async function doPrintAndClose() {
    await nextTick();
    await Promise.race([
        waitForFonts(),
        new Promise<void>((resolve) => setTimeout(resolve, 800)),
    ]);
    await new Promise<void>((resolve) => requestAnimationFrame(() => resolve()));
    await new Promise<void>((resolve) => setTimeout(resolve, 200));

    const closeSoon = () => setTimeout(async () => {
        document.body.classList.remove("print-preview");
        document.documentElement.classList.remove("print-preview");
        try {
            const win = getCurrentWindow();
            if (win.label === "print") {
                await invoke("close_print_window").catch(() => { });
                await win.hide().catch(() => { });
                await win.close().catch(() => { });
                setTimeout(() => {
                    win.destroy().catch(() => { });
                }, 1000);
                return;
            }
        } catch {
            // fall through to route change
        }
        router.replace("/").catch(() => { });
    }, 250);

    window.print();
    closeSoon();

    window.addEventListener("afterprint", closeSoon, { once: true });
    setTimeout(closeSoon, 10000);
}

onMounted(async () => {
    document.body.classList.add("print-preview");
    document.documentElement.classList.add("print-preview");
    invoke("hide_print_window_menu").catch(() => { });
    unlisten = await listen<RaceCardPrintPayload>(PRINT_PAYLOAD_EVENT, async (event) => {
        await handlePayload(event.payload);
    });
    setTimeout(() => {
        emit(PRINT_READY_EVENT, { label: getCurrentWindow().label }).catch(() => { });
    }, 0);

    setTimeout(() => {
        if (!payload.value) {
            getCurrentWindow().close().catch(() => { });
        }
    }, 15000);
});

onBeforeUnmount(() => {
    document.body.classList.remove("print-preview");
    document.documentElement.classList.remove("print-preview");
    if (unlisten) unlisten();
    unlisten = null;
});
</script>

<template>
    <div v-if="payload" class="container print-view">
        <div v-for="race_number in (payload!.printRaces || [])" :key="race_number" class="page">
            <div class="race">
                <header class="header">
                    <RacecardHeader v-if="racecard" :racecard="racecard" :race="race_number" :print="true" />
                </header>

                <main>
                    <RaceDetails :racecard="payload!.raceCard" :race="race_number" :print="true" />
                    <div class="print-divider" aria-hidden="true"></div>
                    <TripAnalysis :race="payload!.raceCard.races[race_number - 1]" :print="true" />
                    <div>
                        <Horse v-for="(horse, idx) in (payload!.raceCard.races[race_number - 1]?.horses || [])"
                            :key="`${race_number}-${horse.id || horse.program_number || horse.post_position || idx}`"
                            :horse="horse"
                            :primePowerComparisons="primePowerComparisonsByRace[race_number] || []"
                            :print="true"></Horse>
                    </div>
                </main>
            </div>
        </div>
    </div>
</template>

<style lang="scss" scoped>
.container {
    width: 100%;
    height: 100%;
    display: block;
    background-color: var(--bg);
    color: var(--fg);
}

.page {
    font-family: MGSans, sans-serif;
    margin: 0 auto;
}

.race {
    break-after: page;
}


.header {
    -webkit-print-color-adjust: exact;
    print-color-adjust: exact;
}

.print-view :deep(.trip-info) {
    font-size: 0.85rem;
}

.print-view :deep(.trip-info-header),
.print-view :deep(.trip-info-row) {
    font-size: 0.85rem;
}

.print-divider {
    margin: 0.5rem 1rem 0.75rem;
    border-top: 2px solid var(--ubuntu-blue);
}

@media print {
    .page {
        break-after: page;
    }

    .page:last-child {
        break-after: auto;
    }

    body,
    .container,
    .page {
        background: #fff;
        color: #000;
    }

    @page {
        size: letter;
        margin: 0.4in;
    }
}
</style>
