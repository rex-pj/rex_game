export interface Permission {
  id: number;
  code: string;
  name: string;
  description: string;
  module: string;
  created_date: string;
  updated_date: string;
}

export interface PermissionRequest {
  id: number;
  code: string;
  name: string;
  description: string;
  module: string;
}
