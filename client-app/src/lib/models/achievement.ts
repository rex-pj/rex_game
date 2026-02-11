export interface Achievement {
  id: number;
  code: string;
  name: string;
  description: string;
  icon: string;
  points: number;
  category: string;
  is_actived: boolean;
  created_date: string;
  updated_date: string;
}

export interface AchievementRequest {
  id: number;
  code: string;
  name: string;
  description: string;
  icon: string;
  points: number;
  category: string;
}
