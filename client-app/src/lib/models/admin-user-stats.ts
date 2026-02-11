export interface AdminUserStats {
  id: number;
  user_id: number;
  user_name: string;
  user_display_name: string;
  total_score: number;
  total_games_played: number;
  total_time_played_seconds: number;
  best_score: number;
  best_combo: number;
  average_accuracy: number;
  current_streak: number;
  best_streak: number;
  last_played_at: string;
}
