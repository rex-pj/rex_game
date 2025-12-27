import type { FlashcardType } from "./flashcard-type";
import type { SelectOption } from "./select-option";

export interface Flashcard {
  id: number;
  name: string;
  description: string;
  sub_description?: string;
  image_id: number;
  flashcard_type_id?: number;
  created_date: string;
  updated_date: string;
  image_url?: string;
}

export interface FlashcardDetail {
  id: number;
  name: string;
  description: string;
  sub_description: string;
  image_id: number;
  created_date: string;
  updated_date: string;
  flashcard_types: FlashcardType[];
}

export interface FlashcardRequest {
  id: number;
  name: string;
  description: string;
  sub_description: string;
  image_data?: File;
  type_ids?: number[];
  types?: SelectOption[];
  file_name?: string;
  content_type?: string;
  image_url?: string;
  original_image_url?: string;
}
