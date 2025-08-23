// api client for communicating with rust backend

import axios from 'axios';
import { ApiResponse, GameState, CreateGameRequest, MakeChoiceRequest } from './types';

const API_BASE = '/api';

// create axios instance with default config
const api = axios.create({
  baseURL: API_BASE,
  headers: {
    'Content-Type': 'application/json',
  },
});

// api client class
export class GameApiClient {
  private currentGameId: string | null = null;

  // create a new game
  async createGame(commanderName: string): Promise<GameState> {
    const request: CreateGameRequest = {
      commander_name: commanderName,
    };

    const response = await api.post<ApiResponse<GameState>>('/game/create', request);
    
    if (!response.data.success || !response.data.data) {
      throw new Error(response.data.error || 'Failed to create game');
    }

    this.currentGameId = response.data.data.game_id;
    return response.data.data;
  }

  // get current game state
  async getGameState(gameId?: string): Promise<GameState> {
    const id = gameId || this.currentGameId;
    if (!id) {
      throw new Error('No game ID available');
    }

    const response = await api.get<ApiResponse<GameState>>(`/game/${id}`);
    
    if (!response.data.success || !response.data.data) {
      throw new Error(response.data.error || 'Failed to get game state');
    }

    return response.data.data;
  }

  // make a choice in the current game
  async makeChoice(choice: number, gameId?: string): Promise<GameState> {
    const id = gameId || this.currentGameId;
    if (!id) {
      throw new Error('No game ID available');
    }

    const request: MakeChoiceRequest = {
      choice,
    };

    const response = await api.post<ApiResponse<GameState>>(`/game/${id}/choice`, request);
    
    if (!response.data.success || !response.data.data) {
      throw new Error(response.data.error || 'Failed to make choice');
    }

    return response.data.data;
  }

  // get current game id
  getCurrentGameId(): string | null {
    return this.currentGameId;
  }

  // set game id (for loading existing games)
  setCurrentGameId(gameId: string): void {
    this.currentGameId = gameId;
  }
}

// create singleton instance
export const gameApi = new GameApiClient();
