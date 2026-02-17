export interface Permission {
  id: number;
  code: string;
  name: string;
  description: string;
  module: string;
  created_on: string;
  updated_on: string;

  assigned?: boolean; // Optional field to indicate if the role is assigned to the user
}

export interface PermissionRequest {
  id: number;
  code: string;
  name: string;
  description: string;
  module: string;
}

export interface PermissionItem {
  id: number;
  name: string;
  code: string;
  module: string;
}
