//////////////////////////////// Provides a simple email builder and several transports.
//////////////////////////////// This mailer contains the available transports for your emails.
//////////////////////////////// Lettre_email provides a simple email builder.
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
//////////////////////////////// Message -> Email message which can be formatted.
//////////////////////////////// StmpTransport -> Sends emails using the SMTP protocol.
//////////////////////////////// Transport -> Represents an Email transport.

pub fn new(to: String , subject : String , body : String) {

    let email = Message::builder()
        .from("rustingwork@gmail.com".parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .body(body)
        .unwrap();

        let creds = Credentials::new("rustingwork@gmail.com".to_string(), "syed@github2023".to_string());

// Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();
    // Open a local connection on port 25
    //let mailer = SmtpTransport::unencrypted_localhost();

    // Send the email

    match mailer.send(&email) {
        Ok(i) => println!("{:?}",i),
        Err(e) => println!("Could not send emails: {:?}", e),
    }
                            //RECEIVING ERROR:
// Sep 19 16:49:53.957 ERROR r2d2: Connection error: Could not connect
// Sep 19 16:49:58.443 ERROR r2d2: Connection error: Could not connect
// Sep 19 16:50:03.297 ERROR r2d2: Connection error: Could not connect
// Sep 19 16:50:08.961 ERROR r2d2: Connection error: Could not connect
// Sep 19 16:50:16.260 ERROR r2d2: Connection error: Could not connect

                    // error return from above Match block:
// Could not send emails: lettre::transport::smtp::Error { kind: Client, source: Error(Some("Connection error: Could not conn
// ect")) }
}
 