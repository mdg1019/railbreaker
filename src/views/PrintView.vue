<script setup lang="ts">
import { onMounted, onBeforeUnmount, ref, nextTick } from "vue";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type { RaceCardPrintPayload } from "../models/print";
import { PRINT_PAYLOAD_EVENT } from "../utils/openPrintWindowEvent";
import "../scss/_main.scss";

const payload = ref<RaceCardPrintPayload | null>(null);
let unlisten: UnlistenFn | null = null;

async function waitForImages() {
    const imgs = Array.from(document.images);
    await Promise.all(
        imgs.map(
            (img) =>
                new Promise<void>((resolve) => {
                    if (img.complete) return resolve();
                    img.addEventListener("load", () => resolve(), { once: true });
                    img.addEventListener("error", () => resolve(), { once: true });
                })
        )
    );
}

async function waitForFonts() {
    const anyDoc = document as any;
    if (anyDoc.fonts?.ready) {
        try { await anyDoc.fonts.ready; } catch { }
    }
}

async function doPrintAndClose() {
    await nextTick();
    await new Promise<void>((r) => requestAnimationFrame(() => r()));
    await waitForFonts();
    await waitForImages();
    await new Promise<void>((r) => requestAnimationFrame(() => r()));

    window.print();

    const win = getCurrentWindow();
    const closeSoon = () => setTimeout(() => win.close().catch(() => { }), 250);

    window.addEventListener("afterprint", closeSoon, { once: true });
    setTimeout(closeSoon, 10000);
}

onMounted(async () => {
    document.body.classList.add("print-preview");
    unlisten = await listen<RaceCardPrintPayload>(PRINT_PAYLOAD_EVENT, async (event) => {
        payload.value = event.payload;
        await doPrintAndClose();
    });

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
    <div class="container">
        <div class="page">
            <header class="hdr">
                <h1 class="title">{{ payload?.raceCard.track ?? "Print" }}</h1>
            </header>

            <main v-if="payload">
                Race Card Goes Here
            </main>
        </div>
    </div>
</template>

<style lang="scss" scoped>
.container {
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    background-color: var(--bg);
    color: var(--fg);
}

.page {
    font-family: MGSans, sans-serif;
    padding: 18px;
}


@media print {
    @page {
        margin: 0.4in;
    }
}
</style>
