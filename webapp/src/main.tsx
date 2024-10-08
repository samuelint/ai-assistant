import './globals.css';
import React from 'react';
import ReactDOM from 'react-dom/client';
import { MainLayout } from './app/main-layout';
import Providers from './providers';
import Routes from './routes';
import { initSentry } from './lib/analytics/sentry';

initSentry();

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <Providers>
      <MainLayout>
        <App />
      </MainLayout>
    </Providers>
  </React.StrictMode>,
);

export default function App() {
  return <Routes />;
}
