export interface FlashcardType {
  id: number;
  name: string;
  description: string;
  is_actived: boolean;
  created_on: string;
  updated_on: string;
}

export interface FlashcardTypeRequest {
  id: number;
  name: string;
  description: string;
}
