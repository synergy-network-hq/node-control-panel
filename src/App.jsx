import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import Layout from './components/Layout';
import SetupWizard from './components/SetupWizard';
import Dashboard from './components/Dashboard';

function App() {
  const [isInitialized, setIsInitialized] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const [nodeInfo, setNodeInfo] = useState(null);

  console.log('App render - isInitialized:', isInitialized, 'isLoading:', isLoading);

  useEffect(() => {
    checkInitialization();
  }, []);

  const checkInitialization = async () => {
    console.log('Starting initialization check...');
    try {
      const initialized = await invoke('check_initialization');
      console.log('Initialization check result:', initialized);
      console.log('Type of result:', typeof initialized);
      setIsInitialized(initialized);
      setIsLoading(false);
    } catch (error) {
      console.error('Initialization check failed:', error);
      console.error('Error details:', JSON.stringify(error));
      setIsInitialized(false);
      setIsLoading(false);
    }
  };

  const handleSetupComplete = (info) => {
    console.log('Setup complete, node info:', info);
    setNodeInfo(info);
    setIsInitialized(true);
  };

  if (isLoading) {
    return (
      <div className="loading-container">
        <div className="spinner"></div>
        <p>Loading Synergy Control Panel...</p>
      </div>
    );
  }

  console.log('Rendering, isInitialized:', isInitialized);

  return (
    <Layout>
      <div style={{ position: 'absolute', top: 0, right: 0, background: 'red', color: 'white', padding: '10px', zIndex: 9999 }}>
        DEBUG: isInitialized={String(isInitialized)}
      </div>
      {!isInitialized ? (
        <SetupWizard onComplete={handleSetupComplete} />
      ) : (
        <Dashboard nodeInfo={nodeInfo} />
      )}
    </Layout>
  );
}

export default App;