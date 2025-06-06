export interface FlashcardType {
  id: number;
  name: string;
  description: string;
  created_date: string;
  updated_date: string;
}

export interface FlashcardTypeRequest {
  id: number;
  name: string;
  description: string;
}
