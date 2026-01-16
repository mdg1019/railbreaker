import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import { GlobalState } from '../models/globalState';

export const useGlobalStateStore = defineStore('globalState', {
  state: () => ({
    globalState: new GlobalState(),
    loading: false,
    error: null as string | null,
  }),
  actions: {
    async loadGlobalState() {
      this.loading = true;
      this.error = null;
      try {
        this.globalState = GlobalState.fromObject(await invoke<any>('load_global_state'));
      } catch (e: any) {
        this.error = e.message || 'Failed to load global state';
      } finally {
        this.loading = false;
      }
    },
    getTracks(): Map<string, string> {
      return this.globalState.tracks;
    },
    getCurrentDirectory(): string {
      return this.globalState.currentDirectory;
    },
    getDownloadsDirectory(): string {
      return this.globalState.downloadsDirectory;
    },
    getRacecardsDirectory(): string {
      return this.globalState.racecardsDirectory;
    }
  }
});
