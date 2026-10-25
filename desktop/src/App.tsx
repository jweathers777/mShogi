import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface DesktopInfo { version: string; engine_available: boolean }
export function App() {
 const [info, setInfo] = useState<DesktopInfo | null>(null);
 const [error, setError] = useState('');
 useEffect(() => { void invoke<DesktopInfo>('desktop_info').then(setInfo).catch((reason: unknown) => setError(String(reason))); }, []);
 return <main className="foundation"><p className="eyebrow">NATIVE DESKTOP · CHESS FIRST</p><h1>mShogi</h1><p>The desktop foundation is ready.</p><p role="status">{error || (info ? `Version ${info.version} · bundled engine ${info.engine_available ? 'available' : 'missing'}` : 'Connecting to the native host…')}</p><p className="muted">The playable board is the next integration step.</p></main>;
}
