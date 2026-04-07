import { useWindowState } from './hooks/useWindowState';
import { WhatsAppView } from './views/WhatsAppView';

export default function App() {
  useWindowState();
  return <WhatsAppView />;
}
