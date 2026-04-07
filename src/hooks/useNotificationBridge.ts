import { useEffect } from 'react';
import { tauriIpc } from '../lib/tauri-ipc';

const TITLE_POLL_INTERVAL_MS = 2000;

function parseUnreadCount(title: string): number {
  const match = /^\((\d+)\)/.exec(title);
  return match ? parseInt(match[1], 10) : 0;
}

export function useNotificationBridge() {
  useEffect(() => {
    let lastCount = 0;

    const syncBadge = () => {
      const count = parseUnreadCount(document.title);
      if (count !== lastCount) {
        lastCount = count;
        tauriIpc.setBadgeCount(count).catch(() => {
          // Badge update failures are non-critical
        });
      }
    };

    // Poll document title periodically since we cannot observe the WebView title directly
    const intervalId = setInterval(syncBadge, TITLE_POLL_INTERVAL_MS);

    return () => {
      clearInterval(intervalId);
    };
  }, []);
}
