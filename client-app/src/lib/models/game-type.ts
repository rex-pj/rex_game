export interface GameType {
  id: number;
  code: string;
  name: string;
  description: string;
  icon: string;
  is_actived: boolean;
  created_on: string;
  updated_on: string;
}

export interface GameTypeRequest {
  id: number;
  code: string;
  name: string;
  description: string;
  icon: string;
}
