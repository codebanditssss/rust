import React, { useState } from 'react';

interface CommanderInputProps {
  onSubmit: (commanderName: string) => void;
}

const CommanderInput: React.FC<CommanderInputProps> = ({ onSubmit }) => {
  const [commanderName, setCommanderName] = useState('');

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const name = commanderName.trim();
    if (name) {
      onSubmit(name);
    }
  };

  return (
    <div className="commander-input">
      <h2>🎮 Welcome to the Rebellion</h2>
      <p>Enter your name to begin commanding the Rebel Alliance</p>
      
      <form onSubmit={handleSubmit}>
        <input
          type="text"
          className="input-field"
          placeholder="Enter your commander name..."
          value={commanderName}
          onChange={(e) => setCommanderName(e.target.value)}
          maxLength={30}
          autoFocus
        />
        <br />
        <button type="submit" className="start-button" disabled={!commanderName.trim()}>
          🚀 Start Mission
        </button>
      </form>
      
      <div style={{ marginTop: '2rem', fontSize: '0.9rem', opacity: 0.8 }}>
        <p>💡 <strong>How to Play:</strong></p>
        <ul style={{ textAlign: 'left', margin: '1rem 0' }}>
          <li>Make strategic decisions that affect the story</li>
          <li>Manage reputation, credits, and Force points</li>
          <li>Complete 4 phases to destroy the Death Star</li>
          <li>Multiple endings based on your choices</li>
        </ul>
      </div>
    </div>
  );
};

export default CommanderInput;
