import { invoke } from '@tauri-apps/api/core';

export interface AudioDevice {
  id: string;
  name: string;
}

export const tauriIpc = {
  setBadgeCount: (count: number) => invoke<void>('set_badge_count', { count }),
  saveWindowState: () => invoke<void>('save_window_state'),
  restoreWindowState: () => invoke<void>('restore_window_state'),
  listAudioInputs: () => invoke<AudioDevice[]>('list_audio_inputs'),
  listAudioOutputs: () => invoke<AudioDevice[]>('list_audio_outputs'),
  setAutostart: (enabled: boolean) => invoke<void>('set_autostart', { enabled }),
};
