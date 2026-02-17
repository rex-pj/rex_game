export interface UserRequest {
  id: number;
  name: string;
  display_name?: string;
  email?: string;
  password?: string;
  password_confirm?: string;
}

export interface ResetPasswordRequest {
  token: string;
  password: string;
  password_confirm?: string;
}

export interface ForgotPasswordRequest {
  email: string;
}

export interface ConfirmUserRequest {
  token: string;
}

export interface UserDto {
  id: number;
  email: string;
  name: string;
  display_name?: string;
  created_by_id?: number;
  created_on: string;
  updated_on: string;
  updated_by_id?: number;
  status_id: number;
}

export interface UserItem {
  id: number;
  name: string;
  user_name: string;
}
