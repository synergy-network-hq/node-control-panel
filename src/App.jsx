import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import Layout from './components/Layout';
import SetupWizard from './components/SetupWizard';
import Dashboard from './components/Dashboard';

function App() {
  const [isInitialized, setIsInitialized] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const [nodeInfo, setNodeInfo] = useState(null);

  useEffect(() => {
    checkInitialization();
  }, []);

  const checkInitialization = async () => {
    try {
      const initialized = await invoke('check_initialization');
      console.log('Initialization check result:', initialized);
      setIsInitialized(initialized);
      setIsLoading(false);
    } catch (error) {
      console.error('Initialization check failed:', error);
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
      {!isInitialized ? (
        <SetupWizard onComplete={handleSetupComplete} />
      ) : (
        <Dashboard nodeInfo={nodeInfo} />
      )}
    </Layout>
  );
}

export default App;