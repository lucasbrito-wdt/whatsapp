import { useState } from 'react';
import { AudioDeviceSelector } from '../components/AudioDeviceSelector';
import { useSettingsStore } from '../store/settings';
import { tauriIpc } from '../lib/tauri-ipc';

interface SettingsViewProps {
  onClose: () => void;
}

export function SettingsView({ onClose }: SettingsViewProps) {
  const autostart = useSettingsStore((s) => s.autostart);
  const setAutostart = useSettingsStore((s) => s.setAutostart);
  const [autostartPending, setAutostartPending] = useState(false);

  async function handleAutostartToggle() {
    const next = !autostart;
    setAutostartPending(true);
    try {
      await tauriIpc.setAutostart(next);
      setAutostart(next);
    } catch {
      // revert is implicit — store value unchanged
    } finally {
      setAutostartPending(false);
    }
  }

  return (
    <>
      {/* Backdrop */}
      <div
        className="absolute inset-0 bg-black/40"
        onClick={onClose}
        aria-hidden="true"
      />

      {/* Panel */}
      <div
        className="absolute top-0 right-0 bottom-0 flex flex-col overflow-hidden shadow-2xl"
        style={{
          width: 400,
          backgroundColor: 'var(--wa-header)',
          animation: 'slideInRight 300ms ease',
        }}
        role="dialog"
        aria-label="Settings"
      >
        {/* Header */}
        <div
          className="flex items-center justify-between px-5 py-4 shrink-0"
          style={{ borderBottom: '1px solid rgba(255,255,255,0.07)' }}
        >
          <h2 className="text-base font-semibold text-white">Settings</h2>
          <button
            onClick={onClose}
            aria-label="Close settings"
            className="flex items-center justify-center w-8 h-8 rounded-full text-white/60 hover:text-white hover:bg-white/10 transition-colors"
          >
            <CloseIcon />
          </button>
        </div>

        {/* Scrollable body */}
        <div className="flex-1 overflow-y-auto px-5 py-4 flex flex-col gap-6">
          {/* Audio devices */}
          <Section title="Audio Devices">
            <AudioDeviceSelector />
          </Section>

          {/* System */}
          <Section title="System">
            <div className="flex items-center justify-between">
              <span className="text-sm text-white/80">Open at login</span>
              <Toggle
                checked={autostart}
                disabled={autostartPending}
                onChange={handleAutostartToggle}
                label="Open at login"
              />
            </div>
          </Section>

          {/* About */}
          <Section title="About">
            <div className="flex flex-col gap-2">
              <p className="text-sm text-white/80">WhatsApp for Linux</p>
              <p className="text-xs text-white/40">Version 1.0.0</p>
              <a
                href="https://github.com"
                target="_blank"
                rel="noopener noreferrer"
                className="text-xs transition-colors"
                style={{ color: 'var(--wa-green)' }}
              >
                View on GitHub
              </a>
            </div>
          </Section>
        </div>
      </div>
    </>
  );
}

interface SectionProps {
  title: string;
  children: React.ReactNode;
}

function Section({ title, children }: SectionProps) {
  return (
    <div className="flex flex-col gap-3">
      <h3 className="text-xs font-semibold text-white/40 uppercase tracking-widest">
        {title}
      </h3>
      {children}
    </div>
  );
}

interface ToggleProps {
  checked: boolean;
  disabled: boolean;
  onChange: () => void;
  label: string;
}

function Toggle({ checked, disabled, onChange, label }: ToggleProps) {
  return (
    <button
      role="switch"
      aria-checked={checked}
      aria-label={label}
      disabled={disabled}
      onClick={onChange}
      className="relative inline-flex items-center h-6 w-11 rounded-full transition-colors focus:outline-none disabled:opacity-50"
      style={{
        backgroundColor: checked ? 'var(--wa-green)' : 'rgba(255,255,255,0.15)',
      }}
    >
      <span
        className="inline-block w-4 h-4 rounded-full bg-white shadow transition-transform duration-200"
        style={{
          transform: checked ? 'translateX(24px)' : 'translateX(4px)',
        }}
      />
    </button>
  );
}

function CloseIcon() {
  return (
    <svg width="14" height="14" viewBox="0 0 14 14" stroke="currentColor" strokeWidth="1.5" aria-hidden="true">
      <line x1="1" y1="1" x2="13" y2="13" />
      <line x1="13" y1="1" x2="1" y2="13" />
    </svg>
  );
}
