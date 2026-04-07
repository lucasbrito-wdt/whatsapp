import { create } from 'zustand';

interface SettingsState {
  audioInputId: string | null;
  audioOutputId: string | null;
  autostart: boolean;
  setAudioInputId: (id: string | null) => void;
  setAudioOutputId: (id: string | null) => void;
  setAutostart: (v: boolean) => void;
}

export const useSettingsStore = create<SettingsState>((set) => ({
  audioInputId: null,
  audioOutputId: null,
  autostart: false,
  setAudioInputId: (id) => set({ audioInputId: id }),
  setAudioOutputId: (id) => set({ audioOutputId: id }),
  setAutostart: (v) => set({ autostart: v }),
}));
