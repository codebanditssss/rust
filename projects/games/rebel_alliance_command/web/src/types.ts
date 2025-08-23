// typescript types matching the rust api structures

export interface GameState {
  game_id: string;
  commander_name: string;
  reputation: number;
  force_points: number;
  credits: number;
  ships_available: number;
  pilots_available: number;
  current_phase: number;
  leia_rescued: boolean;
  death_star_plans: boolean;
  obi_wan_alive: boolean;
  game_over: boolean;
  preparations_made: number;
  current_options: GameOption[];
  phase_description: string;
  last_action_result?: string;
}

export interface GameOption {
  id: number;
  text: string;
  description: string;
  icon: string;
  cost?: number;
  requirement?: string;
  available: boolean;
}

export interface CreateGameRequest {
  commander_name: string;
}

export interface MakeChoiceRequest {
  choice: number;
}

export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
}

export interface GameStats {
  reputation: number;
  force_points: number;
  credits: number;
  ships_available: number;
  pilots_available: number;
  current_phase: number;
}
