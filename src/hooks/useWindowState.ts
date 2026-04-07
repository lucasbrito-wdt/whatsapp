import { useEffect } from 'react';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { tauriIpc } from '../lib/tauri-ipc';

export function useWindowState() {
  useEffect(() => {
    const win = getCurrentWindow();
    const unlistenPromise = win.onCloseRequested(async () => {
      await tauriIpc.saveWindowState();
    });
    return () => {
      unlistenPromise.then((f) => f());
    };
  }, []);
}
