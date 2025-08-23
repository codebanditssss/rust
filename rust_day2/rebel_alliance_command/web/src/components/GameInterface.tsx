import React from 'react';
import { GameState } from '../types';
import StatusPanel from './StatusPanel';
import ChoiceButton from './ChoiceButton';
import GameOverScreen from './GameOverScreen';

interface GameInterfaceProps {
  gameState: GameState;
  onMakeChoice: (choice: number) => void;
  onRestart: () => void;
}

const GameInterface: React.FC<GameInterfaceProps> = ({ 
  gameState, 
  onMakeChoice, 
  onRestart 
}) => {
  // show game over screen if game is finished
  if (gameState.game_over) {
    return <GameOverScreen gameState={gameState} onRestart={onRestart} />;
  }

  return (
    <div className="game-container">
      <StatusPanel gameState={gameState} />
      
      <div className="main-content">
        <div className="phase-description">
          {gameState.phase_description}
        </div>

        {gameState.last_action_result && (
          <div className={`action-result ${gameState.last_action_result.includes('❌') ? 'error' : ''}`}>
            {gameState.last_action_result}
          </div>
        )}

        <div className="choices-container">
          {gameState.current_options.map((option) => (
            <ChoiceButton
              key={option.id}
              option={option}
              onClick={onMakeChoice}
            />
          ))}
        </div>

        {/* phase progress indicator */}
        <div style={{ 
          marginTop: '2rem', 
          textAlign: 'center', 
          fontSize: '0.9rem',
          opacity: 0.7 
        }}>
          Phase {gameState.current_phase} of 4 • Episode IV: A New Hope
        </div>
      </div>
    </div>
  );
};

export default GameInterface;
