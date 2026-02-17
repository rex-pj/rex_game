export interface AdminGameSession {
  id: number;
  user_id: number;
  user_name: string;
  user_display_name: string;
  game_type_code: string;
  game_type_name: string;
  score: number;
  max_score: number;
  accuracy: number;
  time_spent_seconds: number;
  cards_played: number;
  correct_answers: number;
  wrong_answers: number;
  combo_max: number;
  started_at: string;
  completed_at: string;
  created_on: string;
}
