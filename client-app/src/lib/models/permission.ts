export interface Permission {
  id: number;
  code: string;
  name: string;
  description: string;
  module: string;
  created_date: string;
  updated_date: string;

  assigned?: boolean; // Optional field to indicate if the role is assigned to the user
}

export interface PermissionRequest {
  id: number;
  code: string;
  name: string;
  description: string;
  module: string;
}
