use crate::enums::mail_template::MailTemplate;
use rex_game_shared::domain::enums::mail_template_names::MailTemplateNames;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let insert = Query::insert()
            .into_table(MailTemplate::Table)
            .columns([
                MailTemplate::Name,
                MailTemplate::Subject,
                MailTemplate::Body,
                MailTemplate::IsActived,
                MailTemplate::IsEnabled,
                MailTemplate::CreatedOn,
                MailTemplate::UpdatedOn,
            ])
            .values_panic([
                MailTemplateNames::USER_REGISTRATION_CONFIRMATION.into(),
                "Welcome to [platform_name] - Confirm Your Account".into(),
                "<div class='container'>

        <div class='content'>
            <h2>Welcome to [platform_name]!</h2>
            <p>Dear [user_name],</p>
            <p>Thank you for registering with [platform_name]! Your account has been successfully created.</p>
            <p>To activate your account and start enjoying our services, please confirm your email address by clicking the button below:</p>
            <p style='text-align: center;'>
                <a href='[confirmation_url]' class='button'>Confirm Your Email</a>
            </p>
            <p>If the button doesn't work, copy and paste this link into your browser: <br>
            <a href='[confirmation_url]'>[confirmation_url]</a></p>
            <p>This link will expire in [expiration_date].</p>
            <p>We're excited to have you on board!</p>
            <p>Best regards,<br>[platform_name] Team</p>
        </div>

        <!-- Footer -->
        <div class='footer'>
            <p><a href='[platform_url]'>[platform_name]</a></p>
        </div>
    </div>".into(),
                true.into(),
                true.into(),
                chrono::Utc::now().into(),
                chrono::Utc::now().into(),
            ])
            .columns([
                MailTemplate::Name,
                MailTemplate::Subject,
                MailTemplate::Body,
                MailTemplate::IsActived,
                MailTemplate::IsEnabled,
                MailTemplate::CreatedOn,
                MailTemplate::UpdatedOn,
            ])
            .values_panic([
                MailTemplateNames::USER_REGISTRATION_COMPLETION.into(),
                "Welcome to [platform_name] - Your Account is Now Active".into(),
                "<div class='container'>

        <div class='content'>
            <h2>Your Account is Now Active!</h2>
            <p>Dear [user_name],</p>
            <p>Congratulations! Your account with [platform_name] has been successfully activated.</p>
            <p>You can now log in and start exploring all the features we have to offer.</p>
            <p style='text-align: center;'>
                <a href='[login_url]' class='button'>Log In to Your Account</a>
            </p>
            <p>If the button doesn't work, copy and paste this link into your browser: <br>
            <a href='[login_url]'>[login_url]</a></p>
            <p>We're thrilled to have you with us!</p>
            <p>Best regards,<br>[platform_name] Team</p>
        </div>
        <!-- Footer -->
        <div class='footer'>
            <p><a href='[platform_url]'>[platform_name]</a></p>
        </div>
    </div>".into(),
                true.into(),
                true.into(),
                chrono::Utc::now().into(),
                chrono::Utc::now().into(),
            ])
            .columns([
                MailTemplate::Name,
                MailTemplate::Subject,
                MailTemplate::Body,
                MailTemplate::IsActived,
                MailTemplate::IsEnabled,
                MailTemplate::CreatedOn,
                MailTemplate::UpdatedOn,
            ])
            .values_panic([
                MailTemplateNames::PASSWORD_RESET_REQUEST.into(),
                "Password Reset Request for Your [platform_name] Account".into(),
                "<div class='container'>

        <div class='content'>
            <h2>Password Reset Request</h2>
            <p>Dear [user_name],</p>
            <p>We received a request to reset the password for your [platform_name] account.</p>
            <p>To reset your password, please click the button below:</p>
            <p style='text-align: center;'>
                <a href='[reset_url]' class='button'>Reset Your Password</a>
            </p>
            <p>If the button doesn't work, copy and paste this link into your browser: <br>
            <a href='[reset_url]'>[reset_url]</a></p>
            <p>This link will expire in [expiration_date]. If you did not request a password reset, please ignore this email.</p>
            <p>Best regards,<br>[platform_name] Team</p>
        </div>
        <!-- Footer -->
        <div class='footer'>
            <p><a href='[platform_url]'>[platform_name]</a></p>
        </div>
    </div>".into(),
                true.into(),
                true.into(),
                chrono::Utc::now().into(),
                chrono::Utc::now().into(),
            ])
            .columns([
                MailTemplate::Name,
                MailTemplate::Subject,
                MailTemplate::Body,
                MailTemplate::IsActived,
                MailTemplate::IsEnabled,
                MailTemplate::CreatedOn,
                MailTemplate::UpdatedOn,
            ])
            .values_panic([
                MailTemplateNames::PASSWORD_RESET_CONFIRMATION.into(),
                "Your [platform_name] Password Has Been Reset".into(),
                "<div class='container'>

        <div class='content'>
            <h2>Your Password Has Been Reset</h2>
            <p>Dear [user_name],</p>
            <p>Your password for your [platform_name] account has been successfully reset.</p>
            <p>If you did not initiate this change, please contact our support team immediately.</p>
            <p style='text-align: center;'>
                <a href='[login_url]' class='button'>Log In to Your Account</a>
            </p>
            <p>If the button doesn't work, copy and paste this link into your browser: <br>
            <a href='[login_url]'>[login_url]</a></p>
            <p>Best regards,<br>[platform_name] Team</p>
        </div>
        <!-- Footer -->
        <div class='footer'>
            <p><a href='[platform_url]'>[platform_name]</a></p>
        </div>
    </div>".into(),
                true.into(),
                true.into(),
                chrono::Utc::now().into(),
                chrono::Utc::now().into(),
            ])
            .to_owned();
        manager.exec_stmt(insert).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let deleted = Query::delete().from_table(MailTemplate::Table).to_owned();

        manager.exec_stmt(deleted).await?;

        Ok(())
    }
}
