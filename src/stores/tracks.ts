import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';

export const useTracksStore = defineStore('tracks', {
  state: () => ({
    tracks: new Map<string, string>(),
    loading: false,
    error: null as string | null,
  }),
  actions: {
    async loadTracks() {
      this.loading = true;
      this.error = null;
      try {
        const result = await invoke<Record<string, string>>('get_tracks');
        this.tracks = new Map(Object.entries(result));
      } catch (e: any) {
        this.error = e.message || 'Failed to load tracks';
      } finally {
        this.loading = false;
      }
    },
    getTrack(key: string): string | undefined {
      return this.tracks.get(key);
    },
    getAllTracks(): Map<string, string> {
      return this.tracks;
    }
  }
});
