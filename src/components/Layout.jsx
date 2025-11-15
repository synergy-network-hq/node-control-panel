import React from 'react';

function Layout({ children }) {
  return (
    <div className="app-container">
      <header className="app-header">
        <div className="header-content">
          <div className="logo-section">
            <div className="logo-icon">⚡</div>
            <h1>Synergy Control Panel</h1>
          </div>
          <div className="header-info">
            <span className="version-badge">v1.0.0</span>
          </div>
        </div>
      </header>
      <main className="app-main">
        {children}
      </main>
      <footer className="app-footer">
        <p>© 2025 Synergy Network. All rights reserved.</p>
      </footer>
    </div>
  );
}

export default Layout;