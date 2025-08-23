import React from 'react';
import { GameState } from '../types';

interface StatusPanelProps {
  gameState: GameState;
}

const StatusPanel: React.FC<StatusPanelProps> = ({ gameState }) => {
  const getPhaseTitle = (phase: number): string => {
    switch (phase) {
      case 1: return '🚀 Phase 1: Rescue Leia';
      case 2: return '🔍 Phase 2: Decode Plans';
      case 3: return '⚔️ Phase 3: Prepare Battle';
      case 4: return '💥 Phase 4: Death Star Assault';
      default: return '🌟 Mission Complete';
    }
  };

  const getReputationColor = (reputation: number): string => {
    if (reputation >= 80) return '#4caf50';
    if (reputation >= 60) return '#ff9800';
    if (reputation >= 40) return '#ffeb3b';
    return '#f44336';
  };

  return (
    <div className="status-panel">
      <h3 className="status-title">Command Status</h3>
      
      <div className="status-item">
        <span className="status-label">👤 Commander:</span>
        <span className="status-value">{gameState.commander_name}</span>
      </div>

      <div className="status-item">
        <span className="status-label">🎯 Current Phase:</span>
        <span className="status-value">{getPhaseTitle(gameState.current_phase)}</span>
      </div>

      <div className="status-item">
        <span className="status-label">⭐ Reputation:</span>
        <span className="status-value" style={{ color: getReputationColor(gameState.reputation) }}>
          {gameState.reputation}/100
        </span>
      </div>
      <div className="reputation-bar">
        <div 
          className="reputation-fill" 
          style={{ 
            width: `${gameState.reputation}%`,
            background: getReputationColor(gameState.reputation)
          }}
        />
      </div>

      <div className="status-item">
        <span className="status-label">✨ Force Points:</span>
        <span className="status-value">{gameState.force_points}</span>
      </div>

      <div className="status-item">
        <span className="status-label">💰 Credits:</span>
        <span className="status-value">{gameState.credits}</span>
      </div>

      <div className="status-item">
        <span className="status-label">🚀 Ships:</span>
        <span className="status-value">{gameState.ships_available}</span>
      </div>

      <div className="status-item">
        <span className="status-label">👥 Pilots:</span>
        <span className="status-value">{gameState.pilots_available}</span>
      </div>

      <div className="status-item">
        <span className="status-label">👑 Leia Rescued:</span>
        <span className="status-value">
          {gameState.leia_rescued ? '✅ Yes' : '❌ No'}
        </span>
      </div>

      <div className="status-item">
        <span className="status-label">📊 Death Star Plans:</span>
        <span className="status-value">
          {gameState.death_star_plans ? '✅ Complete' : '❌ Incomplete'}
        </span>
      </div>

      <div className="status-item">
        <span className="status-label">🧙‍♂️ Obi-Wan:</span>
        <span className="status-value">
          {gameState.obi_wan_alive ? '✅ Alive' : '💀 Fallen'}
        </span>
      </div>
    </div>
  );
};

export default StatusPanel;
