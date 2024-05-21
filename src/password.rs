use rand::{ thread_rng, rngs::ThreadRng, Rng };
use std::fs;
use std::io::{ self, BufReader, BufRead };

pub enum PasswordType
{
    Password,
    Passphrase
}

pub struct Password{}

impl Password
{
    pub fn generate(password_type: PasswordType) -> String 
    {
        match password_type 
        {
            PasswordType::Password => Password::generate_password(100, (true, true, true, true), false),
            PasswordType::Passphrase => Password::generate_passphrase(3, ('-', true, true))
        }
    }

    fn generate_password(len: u8, options: (bool, bool, bool, bool), ambiguous_char: bool) -> String 
    {
        let small_letters: [char; 26] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
        let capital_letters: [char; 26] = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
        let numbers: [char; 10] = ['0','1','2','3','4','5','6','7','8','9'];
        let symbols: [char; 5] = ['@','#','&','$','*'];

        let options: [bool; 4] = options.into();
        
        let mut rng: ThreadRng = rand::thread_rng();

        let mut password: String = "".to_string();
        
        let mut i: u8 = 0;
        while i < len
        {
            let options_index: usize = Password::get_rand_in_range(&mut rng, options.len() as f32);

            let mut token: char;
            
            match options_index
            {
                0 => token = small_letters[Password::get_rand_in_range(&mut rng, 26.00)],
                1 => token = capital_letters[Password::get_rand_in_range(&mut rng, 26.00)],
                2 => token = numbers[Password::get_rand_in_range(&mut rng, 10.00)],
                3 => token = symbols[Password::get_rand_in_range(&mut rng, 5.0)],
                _ => token = small_letters[Password::get_rand_in_range(&mut rng, 26.00)]
            }

            if ambiguous_char == false 
            {
                match password.chars().last()
                {
                    Some(v) => if v == token { continue;  },
                    _ => () 
                }
            }

            password.push(token);

            i += 1;
        }   

        password
    }

    fn get_rand_in_range(rng: &mut ThreadRng, range: f32) -> usize
    {
       (rng.gen::<f32>() * range) as usize
    }

    fn generate_passphrase(len: u8, options: (char, bool, bool)) -> String 
    {
        let mut file = fs::File::open("words.txt").unwrap();    
        let reader = BufReader::new(file);

        let mut rng: ThreadRng = rand::thread_rng();

        let passphrase: String = String::new();

        let mut i: u8 = 0;
        while i < len 
        {
            let phrase = reader.lines().;

            println!("{:?}", phrase);
        }

        "caio".to_string()
    }
}


