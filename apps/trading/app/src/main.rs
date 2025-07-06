use trading_ops;

extern crate lettre;

use lettre::email::EmailBuilder;
use lettre::transport::smtp::{SecurityLevel, SmtpTransportBuilder};

fn mail() {
    // You might need to enable https://www.google.com/settings/security/lesssecureapps
    // You also might want to enable IMAP and store a copy of all outgoing emails
    // Reference: https://www.digitalocean.com/community/tutorials/how-to-use-google-s-smtp-server
    let to_address = "pprakashs@gmail.com";
    let smtp_server = "smtp.googlemail.com";
    let smtp_username = "pprakashs@gmail";
    let smtp_password = "SmritiMandhana$5562";
    let smtp_port = 587u16;

    let email = EmailBuilder::new()
        .to(to_address)
        .from(smtp_username)
        .subject("I am contacting you in respect of a family treasure of Gold deposited in my name")
        .body("i am Becki Ofori a Ghanian from Ashanti region Kumasi, Ghana.")
        .build()
        .unwrap();

    let mut mailer = SmtpTransportBuilder::new((smtp_server, smtp_port))
        .unwrap()
        .hello_name("localhost")
        .credentials(smtp_username, smtp_password)
        .security_level(SecurityLevel::AlwaysEncrypt)
        .smtp_utf8(true)
        .build();

    let result = mailer.send(email.clone());
    match result {
        Ok(_) => println!("email sent"),
        Err(err) => println!("failed to send email alert: {}", err),
    }
}

fn main() {
    println!("app!");
    //trading_ops::load_securities();

    //trading_ops::read_csv_from_site();

    //trading_ops::process_holdings();

    mail();
}
