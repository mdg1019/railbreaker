<script setup lang="ts">
import { onMounted, onBeforeUnmount, ref, nextTick, computed } from "vue";
import { useRouter } from "vue-router";
import { emit, listen, type UnlistenFn } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type { Racecard } from "../models/racecard";
import type { RaceCardPrintPayload } from "../models/print";
import { PRINT_PAYLOAD_EVENT, PRINT_PAYLOAD_STORAGE_KEY, PRINT_READY_EVENT } from "../utils/openPrintWindowEvent";
import RacecardHeader from "../components/racecard/RacecardHeader.vue";
import RaceDetails from "../components/racecard/RaceDetails.vue";
import Horse from "../components/racecard/Horse.vue";
import "../scss/_main.scss";
import { computePrimePowerComparisons } from "../utils/computePrimePowerComparisons";

const payload = ref<Racecard | RaceCardPrintPayload | null>(null);
const printRaces = ref<number[]>([]);
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
    payload.value = value;
    const raceCount = racecard.value?.races?.length ?? 0;
    printRaces.value = Array.from({ length: raceCount }, (_, idx) => idx + 1);
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
    invoke("hide_print_window_menu").catch(() => { });
    unlisten = await listen<Racecard | RaceCardPrintPayload>(PRINT_PAYLOAD_EVENT, async (event) => {
        await handlePayload(event.payload);
    });
    setTimeout(() => {
        emit(PRINT_READY_EVENT, { label: getCurrentWindow().label }).catch(() => { });
    }, 0);

    try {
        const cached = localStorage.getItem(PRINT_PAYLOAD_STORAGE_KEY);
        if (cached && !payload.value) {
            await handlePayload(JSON.parse(cached));
        }
    } catch {
        // ignore cache errors
    }

    setTimeout(() => {
        if (!payload.value) {
            getCurrentWindow().close().catch(() => { });
        }
    }, 15000);
});

onBeforeUnmount(() => {
    document.body.classList.remove("print-preview");
    if (unlisten) unlisten();
    unlisten = null;
});
</script>

<template>
    <div class="container print-view">
        <div v-for="raceNumber in printRaces" :key="raceNumber" class="page">
            <div class="race">
                <header class="header">
                    <RacecardHeader v-if="racecard" :racecard="racecard" :race="raceNumber" :print="true" />
                </header>

                <main v-if="racecard">
                    <RaceDetails :racecard="racecard" :race="raceNumber" :print="true" />
                    <div>
                        <Horse v-for="(horse, idx) in (racecard.races[raceNumber - 1]?.horses || [])"
                            :key="horse.programNumber || horse.postPosition || idx" :horse="horse"
                            :primePowerComparisons="primePowerComparisonsByRace[raceNumber] || []" :print="true"></Horse>
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
