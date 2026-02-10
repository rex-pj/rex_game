export interface FlashcardType {
  id: number;
  name: string;
  description: string;
  is_actived: boolean;
  created_date: string;
  updated_date: string;
}

export interface FlashcardTypeRequest {
  id: number;
  name: string;
  description: string;
}
