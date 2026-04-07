import { useEffect, useState } from 'react';
import { tauriIpc, AudioDevice } from '../lib/tauri-ipc';
import { useSettingsStore } from '../store/settings';

export function AudioDeviceSelector() {
  const [inputs, setInputs] = useState<AudioDevice[]>([]);
  const [outputs, setOutputs] = useState<AudioDevice[]>([]);
  const [loading, setLoading] = useState(true);

  const audioInputId = useSettingsStore((s) => s.audioInputId);
  const audioOutputId = useSettingsStore((s) => s.audioOutputId);
  const setAudioInputId = useSettingsStore((s) => s.setAudioInputId);
  const setAudioOutputId = useSettingsStore((s) => s.setAudioOutputId);

  useEffect(() => {
    Promise.all([tauriIpc.listAudioInputs(), tauriIpc.listAudioOutputs()])
      .then(([ins, outs]) => {
        setInputs(ins);
        setOutputs(outs);
      })
      .catch(() => {
        // Device enumeration failed — leave lists empty
      })
      .finally(() => setLoading(false));
  }, []);

  if (loading) {
    return (
      <p className="text-sm" style={{ color: 'var(--wa-green)' }}>
        Loading devices...
      </p>
    );
  }

  return (
    <div className="flex flex-col gap-4">
      <DeviceSelect
        label="Microphone"
        devices={inputs}
        value={audioInputId}
        onChange={setAudioInputId}
      />
      <DeviceSelect
        label="Speaker"
        devices={outputs}
        value={audioOutputId}
        onChange={setAudioOutputId}
      />
    </div>
  );
}

interface DeviceSelectProps {
  label: string;
  devices: AudioDevice[];
  value: string | null;
  onChange: (id: string | null) => void;
}

function DeviceSelect({ label, devices, value, onChange }: DeviceSelectProps) {
  return (
    <div className="flex flex-col gap-1">
      <label className="text-xs font-medium text-white/60 uppercase tracking-wide">
        {label}
      </label>
      <select
        value={value ?? ''}
        onChange={(e) => onChange(e.target.value || null)}
        className="w-full rounded px-3 py-2 text-sm text-white outline-none"
        style={{
          backgroundColor: 'var(--wa-bg-dark)',
          border: '1px solid rgba(255,255,255,0.1)',
        }}
      >
        <option value="">System default</option>
        {devices.map((d) => (
          <option key={d.id} value={d.id}>
            {d.name}
          </option>
        ))}
      </select>
    </div>
  );
}
