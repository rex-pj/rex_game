export interface MailTemplate {
  id: number;
  name: string;
  subject: string;
  body: string;
  created_date: string;
  updated_date: string;
  is_active: boolean;
  is_enabled: boolean;
}

export interface MailTemplateRequest {
  id: number;
  name: string;
  subject: string;
  body: string;
}
