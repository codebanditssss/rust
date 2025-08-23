import React from 'react';
import { GameState } from '../types';

interface GameOverScreenProps {
  gameState: GameState;
  onRestart: () => void;
}

const GameOverScreen: React.FC<GameOverScreenProps> = ({ gameState, onRestart }) => {
  const getEndingInfo = () => {
    // check if commander died (assuming alive field exists or check last action result)
    const commanderDied = gameState.last_action_result?.includes('💀') || 
                         gameState.last_action_result?.includes('SACRIFICE');
    
    if (commanderDied) {
      if (gameState.last_action_result?.includes('HEROIC SACRIFICE')) {
        return {
          title: '🏆 HEROIC SACRIFICE',
          message: 'Your ultimate sacrifice saved the entire Rebellion! You will be remembered as the greatest hero of the war!',
          className: 'victory'
        };
      } else {
        return {
          title: '💀 DEFEAT',
          message: 'The Death Star remains operational and the Rebellion has been crushed. The Empire\'s tyranny continues across the galaxy.',
          className: 'defeat'
        };
      }
    }

    if (gameState.last_action_result?.includes('LEGENDARY FORCE VICTORY')) {
      return {
        title: '🌟 LEGENDARY FORCE VICTORY',
        message: 'Your connection to the Force guided the impossible shot! You have become a true Jedi Knight! The galaxy will remember this day forever!',
        className: 'victory'
      };
    }

    if (gameState.last_action_result?.includes('PERFECT VICTORY')) {
      return {
        title: '🎉 PERFECT VICTORY',
        message: 'The Death Star explodes in a brilliant flash! Yavin 4 is saved and the Rebellion lives on! You are hailed as a hero of the galaxy!',
        className: 'victory'
      };
    }

    if (gameState.last_action_result?.includes('COSTLY VICTORY')) {
      return {
        title: '⚔️ COSTLY VICTORY',
        message: 'The Death Star is destroyed but at great cost. Many brave pilots gave their lives for freedom. Their sacrifice will never be forgotten.',
        className: 'victory'
      };
    }

    return {
      title: '🌟 VICTORY',
      message: 'The Rebellion succeeds! The Death Star has been destroyed and freedom is restored to the galaxy!',
      className: 'victory'
    };
  };

  const ending = getEndingInfo();

  return (
    <div className="game-over">
      <h2 className={`game-over-title ${ending.className}`}>
        {ending.title}
      </h2>
      
      <div className="game-over-message">
        {ending.message}
      </div>

      {/* final stats */}
      <div style={{ 
        background: 'rgba(26, 26, 46, 0.8)', 
        border: '1px solid #2196f3', 
        borderRadius: '10px', 
        padding: '1.5rem', 
        margin: '2rem auto', 
        maxWidth: '500px' 
      }}>
        <h3 style={{ marginBottom: '1rem', color: '#2196f3' }}>📊 Final Report</h3>
        <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '0.5rem', fontSize: '0.9rem' }}>
          <div>👤 Commander: {gameState.commander_name}</div>
          <div>⭐ Final Reputation: {gameState.reputation}/100</div>
          <div>✨ Force Mastery: {gameState.force_points} points</div>
          <div>💰 Credits Remaining: {gameState.credits}</div>
          <div>🚀 Ships Saved: {gameState.ships_available}</div>
          <div>👥 Pilots Survived: {gameState.pilots_available}</div>
          <div>👑 Leia Rescued: {gameState.leia_rescued ? '✅' : '❌'}</div>
          <div>📊 Plans Decoded: {gameState.death_star_plans ? '✅' : '❌'}</div>
        </div>
      </div>

      <button className="restart-button" onClick={onRestart}>
        🔄 New Mission
      </button>

      <div style={{ 
        marginTop: '2rem', 
        fontSize: '0.8rem', 
        opacity: 0.7,
        fontStyle: 'italic'
      }}>
        "May the Force be with you, always." ✨
      </div>
    </div>
  );
};

export default GameOverScreen;
