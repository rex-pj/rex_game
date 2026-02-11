export interface GameType {
  id: number;
  code: string;
  name: string;
  description: string;
  icon: string;
  is_actived: boolean;
  created_date: string;
  updated_date: string;
}

export interface GameTypeRequest {
  id: number;
  code: string;
  name: string;
  description: string;
  icon: string;
}
