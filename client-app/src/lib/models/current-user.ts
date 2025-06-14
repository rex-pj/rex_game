export interface CurrentUser {
  display_name: string;
  email: string;
  id: number;
  roles: { role_name: string }[];
}
