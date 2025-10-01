export interface CurrentUser {
  display_name: string;
  name: string;
  email: string;
  id: number;
  roles: string[];
  permissions: string[];
}
