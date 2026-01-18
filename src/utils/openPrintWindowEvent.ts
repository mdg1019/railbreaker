// src/utils/print/openPrintWindowEvent.ts
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { PhysicalPosition } from "@tauri-apps/api/window";
import type { RaceCardPrintPayload } from "../models/print";

export const PRINT_PAYLOAD_EVENT = "print:payload";
export const PRINT_READY_EVENT = "print:ready";
export const PRINT_PAYLOAD_STORAGE_KEY = "print:payload-cache";

type OpenOptions = {
    windowLabel?: string;
    width?: number;
    height?: number;
    timeoutMs?: number;
};

function sleep(ms: number) {
    return new Promise<void>((r) => setTimeout(r, ms));
}

async function waitForPrintReady(label: string, timeoutMs: number) {
    await new Promise<void>(async (resolve) => {
        const timeout = setTimeout(() => resolve(), timeoutMs);
        let unlisten: UnlistenFn | null = null;

        try {
            unlisten = await listen<{ label?: string }>(PRINT_READY_EVENT, (event) => {
                if (event.payload?.label && event.payload.label !== label) {
                    return;
                }
                clearTimeout(timeout);
                if (unlisten) {
                    unlisten();
                }
                resolve();
            });
        } catch {
            clearTimeout(timeout);
            resolve();
        }
    });
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

    let win = await WebviewWindow.getByLabel(label);
    let pageLoaded = Boolean(win);
    const offscreenPosition = new PhysicalPosition(-20000, -20000);

    if (!win) {
        win = new WebviewWindow(label, {
            url,
            title: "",
            width,
            height,
            resizable: true,
            decorations: false,
            focus: false,
            skipTaskbar: true,
            x: offscreenPosition.x,
            y: offscreenPosition.y,
        });

        win.once("tauri://page-load", () => {
            pageLoaded = true;
            win?.setPosition(offscreenPosition).catch(() => { });
        });

        win.once("tauri://error", (e) => {
            console.error("Print window error:", e);
        });
    } else {
        win.setPosition(offscreenPosition).catch(() => { });
    }

    invoke("hide_print_window_menu").catch(() => { });

    const start = Date.now();
    while (!pageLoaded && Date.now() - start < timeoutMs) {
        await sleep(50);
    }

    win.setPosition(offscreenPosition).catch(() => { });

    try {
        if (typeof window !== "undefined" && window.localStorage) {
            window.localStorage.setItem(PRINT_PAYLOAD_STORAGE_KEY, JSON.stringify(payload));
        }
    } catch {
        // best-effort cache for the print window
    }

    await waitForPrintReady(label, Math.min(2000, timeoutMs));
    await win.emit(PRINT_PAYLOAD_EVENT, payload);

    await win.setFocus();
}
