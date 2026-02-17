import type { FlashcardType } from "./flashcard-type";
import type { SelectOption } from "./select-option";

export interface Flashcard {
  id: number;
  name: string;
  description: string;
  sub_description?: string;
  image_id: number;
  flashcard_type_id?: number;
  is_actived: boolean;
  flashcard_type_names: string[];
  created_on: string;
  updated_on: string;
  image_url?: string;
}

export interface FlashcardGameTypeInfo {
  id: number;
  code: string;
  name: string;
}

export interface FlashcardDetail {
  id: number;
  name: string;
  description: string;
  sub_description: string;
  image_id: number;
  created_on: string;
  updated_on: string;
  flashcard_types: FlashcardType[];
  game_types: FlashcardGameTypeInfo[];
}

export interface FlashcardRequest {
  id: number;
  name: string;
  description: string;
  sub_description: string;
  image_data?: File;
  type_ids?: number[];
  types?: SelectOption[];
  game_type_ids?: number[];
  game_types?: SelectOption[];
  file_name?: string;
  content_type?: string;
  image_url?: string;
  original_image_url?: string;
}
