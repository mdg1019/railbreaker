import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import { ConfigState } from '../models/configState';

export const useConfigFileStore = defineStore('configFile', {
  state: () => ({
    configState: new ConfigState(),
    loading: false,
    error: null as string | null,
  }),
  actions: {
    async loadConfigFile() {
      this.loading = true;
      this.error = null;
      try {
        const result = await invoke<any>('load_config_file');
        this.configState = ConfigState.fromObject(result);
      } catch (e: any) {
        this.error = e.message || 'Failed to load config file';
      } finally {
        this.loading = false;
      }
    },
    async saveConfigFile() {
      this.loading = true;
      this.error = null;
      try {
        await invoke('save_config_file', { configState: this.configState.toObject() });
      } catch (e: any) {
        this.error = e.message || 'Failed to save config file';
      } finally {
        this.loading = false;
      }
    },
    getLastDirectory(): string {
      return this.configState.lastDirectory;
    },
    setLastDirectory(directory: string) {
      this.configState.lastDirectory = directory;
    },
    getWindowPosition(): { x: number | null; y: number | null } {
      return { x: this.configState.windowX, y: this.configState.windowY };
    },
    setWindowPosition(x: number, y: number) {
      this.configState.windowX = x;
      this.configState.windowY = y;
    },
    getWindowSize(): { width: number | null; height: number | null } {
      return { width: this.configState.windowWidth, height: this.configState.windowHeight };
    },
    setWindowSize(width: number, height: number) {
      this.configState.windowWidth = width;
      this.configState.windowHeight = height;
    }
  }
});
