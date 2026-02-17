export interface Achievement {
  id: number;
  code: string;
  name: string;
  description: string;
  icon: string;
  points: number;
  category: string;
  is_actived: boolean;
  created_on: string;
  updated_on: string;
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
