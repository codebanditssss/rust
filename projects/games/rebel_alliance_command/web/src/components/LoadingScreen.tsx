import React from 'react';

const LoadingScreen: React.FC = () => {
  return (
    <div className="loading">
      <div className="spinner"></div>
      <p style={{ marginTop: '1rem', fontSize: '1.1rem' }}>
        🌌 Connecting to the Rebellion...
      </p>
    </div>
  );
};

export default LoadingScreen;
