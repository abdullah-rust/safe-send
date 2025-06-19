use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};

use lettre::message::{Mailbox, MultiPart, SinglePart, header};
use std::error::Error;

#[derive(Debug)]
pub enum EmailError {
    InvalidEmail,
    SmtpError(String),
    EmailFormatError,
}

impl std::fmt::Display for EmailError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidEmail => write!(f, "Invalid email address"),
            Self::SmtpError(e) => write!(f, "SMTP error: {}", e),
            Self::EmailFormatError => write!(f, "Email formatting failed"),
        }
    }
}

impl Error for EmailError {}

/// Send verification code via Gmail SMTP
/// Example: send_verification("user@gmail.com", 123456).await?;
pub async fn send_verification(to_email: &str, code: &str) -> Result<(), EmailError> {
    // 1. Validate email format
    if !to_email.contains('@') || !to_email.contains('.') {
        return Err(EmailError::InvalidEmail);
    }

    // 2. Create email (using Gmail's SMTP)
    let email = Message::builder()
        .from(
            "ariaz7556@gmail.com"
                .parse::<Mailbox>()
                .map_err(|_| EmailError::EmailFormatError)?,
        )
        .to(to_email
            .parse::<Mailbox>()
            .map_err(|_| EmailError::InvalidEmail)?)
        .subject("Your Verification Code")
        .multipart(
            MultiPart::alternative() // For both plain text and HTML versions
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_PLAIN)
                        .body(format!("Your verification code is: {}", code)),
                )
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(format!(
                            r#"
                        <html>
                            <body>
                                <h1 style="color: #2563eb;">SafeSend App Verification</h1>
                                <h2>Your code is:<h1>{}</h2> </h2>
                                <p>Valid for 2 minute ⏳</p>
                            </body>
                        </html>
                        "#,
                            code
                        )),
                ),
        )
        .map_err(|_| EmailError::EmailFormatError)?;

    // 3. Gmail SMTP credentials (store these in .env!)
    let creds = Credentials::new(
        "ariaz7556@gmail.com".to_string(),
        "kdja lasi wcbx nwoj".to_string(), // ⚠️ Use App Password (not main Gmail password)
    );

    // 4. Send email (async with Tokio)
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .map_err(|e| EmailError::SmtpError(e.to_string()))?
        .credentials(creds)
        .build();

    mailer
        .send(&email)
        .map_err(|e| EmailError::SmtpError(e.to_string()))?;

    Ok(())
}
