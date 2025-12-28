export interface CurrentUser {
  display_name: string | null;
  name: string;
  email: string;
  id: number;
  roles: string[];
  permissions: string[];
}
