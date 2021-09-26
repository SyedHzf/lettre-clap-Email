mod letre_;
mod clap_;
// use std::{env};

fn main() {
    let mail = clap_::new();
    letre_::new(mail.email,mail.subject,mail.context);

 
}