///////////////// clap is a simple-to-use, efficient, and full-featured library 
/////////// for parsing command line arguments and subcommands when writing
////// console/terminal applications.
use clap::{Arg,App};
///////////////// Arg -> Used to set all the options and relationships 
//////////////that define a valid argument for the program.
//////////////// App -> Represents a command line interface which is made
////////////// up of all possible command line arguments and subcommands.
pub struct Mail{
  pub email: String,
    pub subject: String,
    pub context: String,
} 

pub fn new() -> Mail{
// app name 
    let matches = App::new("Pegham")
// version
    .version("0.1.0")
//author
    .author("S_H_A_H")
// about the app
    .about("Ek baar azmaish Bar bar frmaish!")
// Email flag
    .arg(Arg::new("Email")
            .long("email") //
            .short('e')
            .value_name("EMAIL")
            .about("Write the email address")
            .takes_value(true)
        )
// Subject Flag
        .arg(Arg::new("Subject")
            .long("subject")
            .short('s')
            .value_name("SUBJECT")
            .about("Write the subject")
            .takes_value(true)
        )
// Content Flag
        .arg(Arg::new("Content")
            .long("content")
            .short('c')
            .value_name("CONTENT")
            .about("Write the content")
            .takes_value(true)
        )


        .get_matches();

    Mail{
        email:matches.value_of("Email").unwrap().to_string(),
        subject:matches.value_of("Subject").unwrap().to_string(),
        context:matches.value_of("Content").unwrap().to_string(),
    }



}
