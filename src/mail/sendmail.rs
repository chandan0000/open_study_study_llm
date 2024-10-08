use dotenvy_macro::dotenv;
use lettre::{
    message::{header, SinglePart},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};
use std::fs;

pub async fn send_email(
    to_email: &str,
    subject: &str,
    template_path: &str,
    placeholders: &[(String, String)],
) -> Result<(), Box<dyn std::error::Error>> {
    // Fetch SMTP credentials from environment variables
    let smtp_username = dotenv!("SMTP_USERNAME");
    let smtp_password = dotenv!("SMTP_PASSWORD");
    let smtp_server = dotenv!("SMTP_SERVER");
    let smtp_port: u16 = dotenv!("SMTP_PORT").parse()?;

    // Read and customize the email template
    let mut html_template = fs::read_to_string(template_path)?;
    for (key, value) in placeholders {
        html_template = html_template.replace(key, value);
    }


    // Include a custom "From" header with display name
    let from_header = format!("OpenStudyIndia <{}>", smtp_username);

    // Build the email
    let email = Message::builder()
        .from(from_header.parse()?) // Include custom display name in "From" header
        .to(to_email.parse()?)
        .subject(subject)
        .header(header::ContentType::TEXT_HTML)
        .singlepart(
            SinglePart::builder()
                .header(header::ContentType::TEXT_HTML)
                .body(html_template),
        )?;

    // SMTP credentials
    let creds = Credentials::new(
        smtp_username.to_string().clone(),
        smtp_password.to_string().clone(),
    );

    // Set up the mailer with STARTTLS
    let mailer = SmtpTransport::starttls_relay(&smtp_server)?
        .credentials(creds)
        // .port(smtp_port)
        .build();

    // Attempt to send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => println!("Failed to send email: {:?}", e),
    }

    Ok(())
}
