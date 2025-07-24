use lettre::message::header::ContentType;
use lettre::message::MessageBuilder;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::response::Response;
use lettre::{SmtpTransport, Transport};
use std::error::Error;
use tera::{Context, Tera};

pub struct HtmlMailer {
    pub template_engine: Tera,
    pub smtp_host: String,
    pub smtp_username: String,
    pub smtp_password: String,
    pub default_subject: String,
}

impl HtmlMailer {
    pub fn builder() -> HtmlMailerBuilder {
        HtmlMailerBuilder::default()
    }

    pub fn send(
        &self,
        to: String,
        template_name: &str,
        template_context: Context,
    ) -> Result<Response, Box<dyn Error>> {
        self.send_with_subject(
            to,
            self.default_subject.clone(),
            template_name,
            template_context,
        )
    }

    pub fn send_with_subject(
        &self,
        to: String,
        subject: String,
        template_name: &str,
        template_context: Context,
    ) -> Result<Response, Box<dyn Error>> {
        let html_body = self
            .template_engine
            .render(template_name, &template_context)?;

        let message = MessageBuilder::new()
            .subject(subject)
            .from("Cr8s <noreply@cr8s.com>".parse()?)
            .to(to.parse()?)
            .header(ContentType::TEXT_HTML)
            .body(html_body)?;

        let credentials = Credentials::new(self.smtp_username.clone(), self.smtp_password.clone());
        let mailer = SmtpTransport::relay(&self.smtp_host)?
            .credentials(credentials)
            .build();

        mailer.send(&message).map_err(|e| e.into())
    }
}

pub struct HtmlMailerBuilder {
    template_engine: Option<Tera>,
    smtp_host: Option<String>,
    smtp_username: Option<String>,
    smtp_password: Option<String>,
    default_subject: String,
}

impl Default for HtmlMailerBuilder {
    fn default() -> Self {
        Self {
            template_engine: None,
            smtp_host: None,
            smtp_username: None,
            smtp_password: None,
            default_subject: "Cr8s digest".to_string(),
        }
    }
}

impl HtmlMailerBuilder {
    pub fn default_subject(mut self, subject: String) -> Self {
        self.default_subject = subject;
        self
    }

    pub fn build(self) -> HtmlMailer {
        HtmlMailer {
            template_engine: self.template_engine.expect("template_engine is required"),
            smtp_host: self.smtp_host.expect("smtp_host is required"),
            smtp_username: self.smtp_username.expect("smtp_username is required"),
            smtp_password: self.smtp_password.expect("smtp_password is required"),
            default_subject: self.default_subject,
        }
    }
    pub fn template_engine(mut self, template_engine: Tera) -> Self {
        self.template_engine = Some(template_engine);
        self
    }

    pub fn smtp_host(mut self, smtp_host: String) -> Self {
        self.smtp_host = Some(smtp_host);
        self
    }

    pub fn smtp_username(mut self, smtp_username: String) -> Self {
        self.smtp_username = Some(smtp_username);
        self
    }

    pub fn smtp_password(mut self, smtp_password: String) -> Self {
        self.smtp_password = Some(smtp_password);
        self
    }
}
