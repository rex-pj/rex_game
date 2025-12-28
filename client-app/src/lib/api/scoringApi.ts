import { BaseApi } from "./baseApi";
import type { BaseApiOptions } from "./apiOptions";

export interface GameType {
  id: number;
  code: string;
  name: string;
  description?: string;
  icon?: string;
}

export interface StartGameSessionRequest {
  game_type_code: string;
  flashcard_type_id?: number;
}

export interface CompleteGameSessionRequest {
  session_id: number;
  score: number;
  max_score?: number;
  correct_answers: number;
  wrong_answers: number;
  combo_max: number;
  time_spent_seconds: number;
}

export interface GameSession {
  id: number;
  game_type_code?: string;
  game_type_name?: string;
  score: number;
  max_score?: number;
  accuracy?: number;
  time_spent_seconds: number;
  cards_played: number;
  correct_answers: number;
  wrong_answers: number;
  combo_max: number;
  completed_at?: string;
}

export interface UserStats {
  user_id: number;
  user_name?: string;
  user_display_name?: string;
  total_score: number;
  total_games_played: number;
  total_time_played_seconds: number;
  best_score: number;
  best_combo: number;
  average_accuracy: number;
  current_streak: number;
  best_streak: number;
  rank?: number;
}

export interface LeaderboardEntry {
  rank: number;
  user_id: number;
  user_name: string;
  user_display_name?: string;
  total_score: number;
  total_games_played: number;
  best_score: number;
  average_accuracy: number;
}

export interface Achievement {
  id: number;
  code: string;
  name: string;
  description?: string;
  icon?: string;
  points: number;
  category?: string;
  unlocked: boolean;
  unlocked_at?: string;
}

export interface GameCompleteResponse {
  session: GameSession;
  new_achievements: Achievement[];
  updated_stats: UserStats;
}

export interface GameProgress {
  id: number;
  user_id: number;
  game_type_id: number;
  game_type_code?: string;
  game_type_name?: string;
  current_level: number;
  highest_level: number;
  total_score: number;
  last_played_at: string;
}

export interface SaveGameProgressRequest {
  game_type_code: string;
  current_level: number;
  total_score: number;
}

export class ScoringApi extends BaseApi {
  constructor(options: BaseApiOptions) {
    super(options);
  }

  // Public endpoints
  async getGameTypes(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>
  ): Promise<GameType[]> {
    return await this.get(fetch, "/game-types", new URLSearchParams());
  }

  async getLeaderboard(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    page: number = 1,
    pageSize: number = 10
  ): Promise<LeaderboardEntry[]> {
    const params = new URLSearchParams();
    params.set("page", page.toString());
    params.set("page_size", pageSize.toString());
    return await this.get(fetch, "/leaderboard", params);
  }

  async getAchievements(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>
  ): Promise<Achievement[]> {
    return await this.get(fetch, "/achievements", new URLSearchParams());
  }

  async getUserStats(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    userId: number
  ): Promise<UserStats | null> {
    return await this.get(fetch, `/users/${userId}/stats`, new URLSearchParams());
  }

  // Authenticated endpoints
  async startGameSession(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    request: StartGameSessionRequest
  ): Promise<number> {
    return await this.post(fetch, "/games/sessions", request);
  }

  async completeGameSession(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    request: CompleteGameSessionRequest
  ): Promise<GameCompleteResponse> {
    return await this.post(fetch, "/games/sessions/complete", request);
  }

  async getGameHistory(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    page: number = 1,
    pageSize: number = 10
  ): Promise<GameSession[]> {
    const params = new URLSearchParams();
    params.set("page", page.toString());
    params.set("page_size", pageSize.toString());
    return await this.get(fetch, "/games/history", params);
  }

  async getBestGames(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    gameType?: string,
    limit: number = 10
  ): Promise<GameSession[]> {
    const params = new URLSearchParams();
    if (gameType) params.set("game_type", gameType);
    params.set("limit", limit.toString());
    return await this.get(fetch, "/games/best", params);
  }

  async getMyStats(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>
  ): Promise<UserStats | null> {
    return await this.get(fetch, "/users/me/stats", new URLSearchParams());
  }

  async getMyAchievements(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>
  ): Promise<Achievement[]> {
    return await this.get(fetch, "/users/me/achievements", new URLSearchParams());
  }

  // Game Progress endpoints
  async getGameProgress(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    gameTypeCode: string
  ): Promise<GameProgress | null> {
    const params = new URLSearchParams();
    params.set("game_type", gameTypeCode);
    return await this.get(fetch, "/games/progress", params);
  }

  async saveGameProgress(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    request: SaveGameProgressRequest
  ): Promise<GameProgress> {
    return await this.post(fetch, "/games/progress", request);
  }

  async resetGameProgress(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    gameTypeCode: string
  ): Promise<void> {
    const params = new URLSearchParams();
    params.set("game_type", gameTypeCode);
    return await this.delete(fetch, "/games/progress", params);
  }
}
