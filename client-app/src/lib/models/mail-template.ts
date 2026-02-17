export interface MailTemplate {
  id: number;
  name: string;
  subject: string;
  body: string;
  created_on: string;
  updated_on: string;
  is_active: boolean;
  is_enabled: boolean;
}

export interface MailTemplateRequest {
  id: number;
  name: string;
  subject: string;
  body: string;
}
