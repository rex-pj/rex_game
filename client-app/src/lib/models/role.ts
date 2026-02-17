export interface Role {
  id: number;
  name: string;
  description: string;
  created_on: string;
  updated_on: string;
  assigned?: boolean; // Optional field to indicate if the role is assigned to the user
}

export interface RoleRequest {
  id: number;
  name: string;
  description: string;
}

export interface RoleItem {
  id: number;
  name: string;
}
