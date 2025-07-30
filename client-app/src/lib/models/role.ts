export interface Role {
  id: number;
  name: string;
  description: string;
  created_date: string;
  updated_date: string;
  assigned?: boolean; // Optional field to indicate if the role is assigned to the user
}

export interface RoleRequest {
  id: number;
  name: string;
  description: string;
}
