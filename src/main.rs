use lettre::{
    message::{header::ContentType, Attachment, Body, Mailbox, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};

fn send_email() {
    let email = Message::builder()
        .from("contact@bocksdincoding.com".parse::<Mailbox>().unwrap())
        .to("questions@bocksdincoding.com".parse::<Mailbox>().unwrap())
        .subject("Test Email")
        .body("Hello, this is a test email!".to_string())
        .unwrap();

    let username = std::env::var("EMAIL_USERNAME").expect("EMAIL_USERNAME not set");
    let password = std::env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD not set");

    let creds = Credentials::new(username, password);

    let mailer = SmtpTransport::relay("mail.privateemail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Basic email sent!"),
        Err(error) => {
            println!("Basic email failed to send. {:?}", error);
        }
    }
}

fn send_email_with_html() {
    let email = Message::builder()
        .from("contact@bocksdincoding.com".parse::<Mailbox>().unwrap())
        .to("questions@bocksdincoding.com".parse::<Mailbox>().unwrap())
        .subject("Test Email")
        .header(ContentType::TEXT_HTML)
        .body(
            "<h1>Hello, this is a test email!</h1>
            <p>This is additional context.</p>
            <a href=\"https://bocksdincoding.com\">Check out my blog!</a>"
                .to_string(),
        )
        .unwrap();

    let username = std::env::var("EMAIL_USERNAME").expect("EMAIL_USERNAME not set");
    let password = std::env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD not set");

    let creds = Credentials::new(username, password);

    let mailer = SmtpTransport::relay("mail.privateemail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email with HTML sent!"),
        Err(error) => {
            println!("Email with HTML failed to send. {:?}", error);
        }
    }
}

fn send_email_with_attachments() {
    let image = std::fs::read("src/logo.png").unwrap();
    let image_body = Body::new(image);

    let email = Message::builder()
        .from("contact@bocksdincoding.com".parse::<Mailbox>().unwrap())
        .to("questions@bocksdincoding.com".parse::<Mailbox>().unwrap())
        .subject("Test Email")
        .multipart(
            MultiPart::related()
                .singlepart(SinglePart::html(
                    "<img src=\"cid:logo.png\" height=50 width=50 />
                <h1>Hello, this is a test email!</h1>
                <p>This is additional context.</p>
                <a href=\"https://bocksdincoding.com\">Check out my blog!</a>"
                        .to_string(),
                ))
                .singlepart(
                    Attachment::new_inline(String::from("logo.png"))
                        .body(image_body, "image/png".parse().unwrap()),
                ),
        )
        .unwrap();

    let username = std::env::var("EMAIL_USERNAME").expect("EMAIL_USERNAME not set");
    let password = std::env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD not set");

    let creds = Credentials::new(username, password);

    let mailer = SmtpTransport::relay("mail.privateemail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email with attachments sent!"),
        Err(error) => {
            println!("Email with attachments failed to send. {:?}", error);
        }
    }
}

fn send_email_with_multiple_recipients() {
    let recipients = vec![
        "questions@bocksdincoding.com",
        "Rory <rory@bocksdincoding.com>",
        // ...
    ];

    let mut email = Message::builder()
        .from("contact@bocksdincoding.com".parse::<Mailbox>().unwrap())
        .subject("Test Email");

    for recipient in recipients {
        email = email.to(recipient.parse::<Mailbox>().unwrap());
    }

    let email = email
        .body("Hello, this is a test email!".to_string())
        .unwrap();

    let username = std::env::var("EMAIL_USERNAME").expect("EMAIL_USERNAME not set");
    let password = std::env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD not set");

    let creds = Credentials::new(username, password);

    let mailer = SmtpTransport::relay("mail.privateemail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email with multiple recipients sent!"),
        Err(error) => {
            println!("Email with multiple recipients failed to send. {:?}", error);
        }
    }
}

fn main() {
    dotenv::dotenv().ok();
    println!("Sending emails...");
    send_email();
    send_email_with_html();
    send_email_with_attachments();
    send_email_with_multiple_recipients();
    println!("Emails sent!");
}
