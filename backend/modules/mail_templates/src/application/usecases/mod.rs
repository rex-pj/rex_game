pub mod mail_template_creation_dto;
pub mod mail_template_deletion_dto;
pub mod mail_template_dto;
pub mod mail_template_updation_dto;
pub mod mail_template_usecase;
pub mod mail_template_usecase_trait;

pub use mail_template_creation_dto::MailTemplateCreationDto;
pub use mail_template_deletion_dto::MailTemplateDeletionDto;
pub use mail_template_dto::MailTemplateDto;
pub use mail_template_updation_dto::MailTemplateUpdationDto;
pub use mail_template_usecase::MailTemplateUseCase;
pub use mail_template_usecase_trait::MailTemplateUseCaseTrait;
