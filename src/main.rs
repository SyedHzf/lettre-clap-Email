use clap::{AppSettings, Clap};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

const USER_NAME: &'static str = "USER_NAME";
const USER_EMAIL: &'static str = "USER_EMAIL";
const PASSWORD: &'static str = "PASSWORD";
const EMAIL_SERVER: &'static str = "smtp.gmail.com";

#[derive(Clap)]

#[clap(setting = AppSettings::ColoredHelp)]
struct Email {
    #[clap(short, long)]
    name: String,
    /// Email address to send email.
    #[clap(short, long)]
    email: String,

    /// Subject Line for email
    #[clap(short, long, default_value = "Yeh Email apko CLI sy musool hoi hain.")]
    subject: String,

    /// Content for Email body
    #[clap(short, long, default_value = "Email jism khali hai.")]
    body: String,
}

fn main() -> anyhow::Result<()> {
    
    // Sender Info from Env vars or secrets
    let sender_name = dotenv::var(USER_NAME)?;
    let sender_email = dotenv::var(USER_EMAIL)?;
    let sender_passwd = dotenv::var(PASSWORD)?;
    println!("{}",sender_email);

    // Parse Cli opts
    let args = Email::parse();

    // Create email.
    let email = Message::builder()
        .from(
            format!("{} <{}>", sender_name, sender_email)
                .parse()
                .unwrap(),
        )
        .to(format!("{} <{}>", &args.name, &args.email).parse().unwrap())
        .subject(&args.subject)
        .body(args.body.clone())
        .unwrap();

        // Open a remote connection to gmail
//     let mailer = SmtpTransport::relay("smtp.gmail.com")
//         .unwrap()
//         .credentials(creds)
//         .build();
//     // Open a local connection on port 25
//     //let mailer = SmtpTransport::unencrypted_localhost();

    // Add credentials

    let creds = Credentials::new(sender_email, sender_passwd);

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(EMAIL_SERVER)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => println!("Couldn't Send Email : {:#?}", e),
    }

    Ok(())
}
// pub fn new() -> Mail{

// app name 
//     let matches = App::new("Pegham")
// // version
//     .version("0.1.0")
// //author
//     .author("S_H_A_H")
// // about the app
//     .about("Ek baar azmaish Bar bar frmaish!")
// // Email flag
//     .arg(Arg::new("Email")
//             .long("email") //
//             .short('e')
//             .value_name("EMAIL")
//             .about("Write the email address")
//             .takes_value(true)
//         )
// // Subject Flag
//         .arg(Arg::new("Subject")
//             .long("subject")
//             .short('s')
//             .value_name("SUBJECT")
//             .about("Write the subject")
//             .takes_value(true)
//         )
// // Content Flag
//         .arg(Arg::new("Content")
//             .long("content")
//             .short('c')
//             .value_name("CONTENT")
//             .about("Write the content")
//             .takes_value(true)
//         )


//         .get_matches();

//     Mail{
//         email:matches.value_of("Email").unwrap().to_string(),
//         subject:matches.value_of("Subject").unwrap().to_string(),
//         context:matches.value_of("Content").unwrap().to_string(),
//     }
//}