// src/utils/print/openPrintWindowEvent.ts
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import type { RaceCardPrintPayload } from "../models/print";

export const PRINT_PAYLOAD_EVENT = "print:payload";

type OpenOptions = {
    windowLabel?: string;
    width?: number;
    height?: number;
    timeoutMs?: number;
};

function sleep(ms: number) {
    return new Promise<void>((r) => setTimeout(r, ms));
}

export async function openPrintWindowAndSendPayload(
    payload: RaceCardPrintPayload,
    opts: OpenOptions = {},
) {
    const label = opts.windowLabel ?? "print";
    const url = "#/print";
    const width = opts.width ?? 816;
    const height = opts.height ?? 1056;
    const timeoutMs = opts.timeoutMs ?? 8000;

    const win = new WebviewWindow(label, {
        url,
        title: "",
        width,
        height,
        resizable: true,
        decorations: true,
        focus: true,
    });

    let pageLoaded = false;

    win.once("tauri://page-load", () => {
        pageLoaded = true;
    });

    win.once("tauri://error", (e) => {
        console.error("Print window error:", e);
    });

    const start = Date.now();
    while (!pageLoaded && Date.now() - start < timeoutMs) {
        await sleep(50);
    }

    await win.emit(PRINT_PAYLOAD_EVENT, payload);

    await win.setFocus();
}
