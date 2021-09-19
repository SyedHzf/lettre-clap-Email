extern crate clap;
// use clap::{Arg,App};


// mod Confiq; 
mod letre_;
mod clap_;
// use std::{env};

fn main() {
    let mail = clap_::new();
    letre_::new(mail.email,mail.subject,mail.context);

 
    
//     //
    // clap_::new();
//     let args : Vec<String> = env::args().collect();
//     //
//     let confiq = Confiq::Confiq::new(&args);
//     //
//     println!("{:?}  {:?} {:?}", confiq.email, confiq.subject, confiq.text);
//     //
//    let result =  letre_::new(
//         confiq.email,
//          confiq.subject,  
//          confiq.text
//         );
// print!("{:?}",result );
}