import { useState } from 'react';
import { TitleBar } from '../components/TitleBar';
import { SettingsView } from './SettingsView';
import { useNotificationBridge } from '../hooks/useNotificationBridge';

export function WhatsAppView() {
  const [settingsOpen, setSettingsOpen] = useState(false);

  useNotificationBridge();

  return (
    <div
      className="flex flex-col w-full h-full"
      style={{ pointerEvents: 'none' }}
    >
      {/* Title bar always captures pointer events */}
      <div style={{ pointerEvents: 'auto' }}>
        <TitleBar />
      </div>

      {/* Overlay layer — fills remaining space above the WebView */}
      <div className="relative flex-1">
        {/* Settings toggle button */}
        <button
          onClick={() => setSettingsOpen(true)}
          aria-label="Open settings"
          style={{ pointerEvents: 'auto', backgroundColor: 'var(--wa-teal)' }}
          className="absolute bottom-4 right-4 flex items-center justify-center w-10 h-10 rounded-full shadow-lg transition-transform hover:scale-110 active:scale-95 focus:outline-none"
        >
          <SettingsGearIcon />
        </button>

        {/* Settings panel */}
        {settingsOpen && (
          <div
            className="absolute inset-0"
            style={{ pointerEvents: 'auto' }}
          >
            <SettingsView onClose={() => setSettingsOpen(false)} />
          </div>
        )}
      </div>
    </div>
  );
}

function SettingsGearIcon() {
  return (
    <svg
      width="18"
      height="18"
      viewBox="0 0 24 24"
      fill="none"
      stroke="white"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
      aria-hidden="true"
    >
      <circle cx="12" cy="12" r="3" />
      <path d="M19.4 15a1.65 1.65 0 00.33 1.82l.06.06a2 2 0 010 2.83 2 2 0 01-2.83 0l-.06-.06a1.65 1.65 0 00-1.82-.33 1.65 1.65 0 00-1 1.51V21a2 2 0 01-4 0v-.09A1.65 1.65 0 009 19.4a1.65 1.65 0 00-1.82.33l-.06.06a2 2 0 01-2.83-2.83l.06-.06A1.65 1.65 0 004.68 15a1.65 1.65 0 00-1.51-1H3a2 2 0 010-4h.09A1.65 1.65 0 004.6 9a1.65 1.65 0 00-.33-1.82l-.06-.06a2 2 0 012.83-2.83l.06.06A1.65 1.65 0 009 4.68a1.65 1.65 0 001-1.51V3a2 2 0 014 0v.09a1.65 1.65 0 001 1.51 1.65 1.65 0 001.82-.33l.06-.06a2 2 0 012.83 2.83l-.06.06A1.65 1.65 0 0019.4 9a1.65 1.65 0 001.51 1H21a2 2 0 010 4h-.09a1.65 1.65 0 00-1.51 1z" />
    </svg>
  );
}
