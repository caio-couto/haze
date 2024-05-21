#![allow(warnings)]

mod valt;
mod secret;
mod password;

use secret::Secret;

use crate::password::Password;

fn main()
{
    let secret1: Secret = Secret::new("google".to_string(), "Conta google".to_string(), "caiokj517@gmail.com".to_string(), "caio123".to_string(), true);

    println!("{:?}", secret1);

    let password1: String = Password::generate(password::PasswordType::Passphrase);
}
