export interface Role {
  id: number;
  name: string;
  description: string;
  created_date: string;
  updated_date: string;
}

export interface RoleRequest {
  id: number;
  name: string;
  description: string;
}
