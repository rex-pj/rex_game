export interface UserRequest {
  id: number;
  name: string;
  display_name?: string;
  email?: string;
}

export interface UserDto {
  id: number;
  email: string;
  name: string;
  display_name?: string;
  created_by_id?: number;
  created_date: string;
  updated_date: string;
  updated_by_id?: number;
  status_id: number;
}
