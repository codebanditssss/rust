import React, { useState, useEffect } from 'react';
import { GameState } from './types';
import { gameApi } from './api';
import CommanderInput from './components/CommanderInput';
import GameInterface from './components/GameInterface';
import LoadingScreen from './components/LoadingScreen';
import './App.css';

function App() {
  const [gameState, setGameState] = useState<GameState | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // create new game
  const handleCreateGame = async (commanderName: string) => {
    setLoading(true);
    setError(null);
    
    try {
      const newGame = await gameApi.createGame(commanderName);
      setGameState(newGame);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create game');
    } finally {
      setLoading(false);
    }
  };

  // make a choice in the game
  const handleMakeChoice = async (choice: number) => {
    if (!gameState) return;
    
    setLoading(true);
    setError(null);
    
    try {
      const updatedGame = await gameApi.makeChoice(choice);
      setGameState(updatedGame);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to make choice');
    } finally {
      setLoading(false);
    }
  };

  // restart game
  const handleRestart = () => {
    setGameState(null);
    setError(null);
  };

  return (
    <div className="app">
      <h1 className="game-title">
        ⭐ STAR WARS: REBEL COMMAND ⭐
      </h1>
      
      {error && (
        <div className="error-message">
          ❌ {error}
          <button onClick={() => setError(null)} className="error-dismiss">
            Dismiss
          </button>
        </div>
      )}

      {loading && <LoadingScreen />}

      {!gameState && !loading && (
        <CommanderInput onSubmit={handleCreateGame} />
      )}

      {gameState && !loading && (
        <GameInterface 
          gameState={gameState}
          onMakeChoice={handleMakeChoice}
          onRestart={handleRestart}
        />
      )}
    </div>
  );
}

export default App;
