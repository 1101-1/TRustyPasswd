use rand::{self, Rng};

pub fn generate_passwd() -> String {
    let mut pass: Vec<char> = Vec::with_capacity(12);
    let chars: [char; 52] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    let numbers: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
    let specific_symbols: [char; 3] = ['@', '!', '#'];

    for _ in 0..=11 {
        let rand_num = rand::thread_rng().gen_range(1..=3);
        match rand_num {
            1 => {
                pass.push(chars[rand::thread_rng().gen_range(0..52)]);
            }
            2 => {
                pass.push(numbers[rand::thread_rng().gen_range(0..10)]);
            }
            3 => {
                pass.push(specific_symbols[rand::thread_rng().gen_range(0..2)]);
            }
            _ => {
                "Err".to_string();
            }
        }
    }

    pass.iter().collect::<String>()
}