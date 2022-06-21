use std::env;
use std::fs;
use std::vec;


fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 { // equivalent to two "actual" command line arguments
        println!("Please call with two command line arguments, the first being a file to search and the second being a string to search for.");
    } 

    else {
        let filename = &args[1];
        let search_string = &args[2];

        let file_contents = fs::read_to_string(filename)
            .expect("Invalid file name");

        let table = build_table(String::from(search_string));
        
        for i in table {
            print!("{} ", i);
        }
    }
}



fn search(text: &str, search_string: &str) -> i32 {
    let table = build_table(search_string.to_string());

    let search_string_chars: Vec<char> = search_string.chars().collect();
    let text_chars: Vec<char> = text.chars().collect();
    let search_string_length = search_string.len();
    let text_length = text.len();

    let mut k = 0;
    let mut i = 0;

    while k + i < text_length {
        if search_string_chars[i] == text_chars[k + i] {
            i += 1;
            if i == search_string_length {
                return k.try_into().unwrap()
            }
        } else if table[i] == -1 {
            k = k + i + 1;
            i = 0;
        } else {
            k = k + i - table[i];
            i = table[i];
        }
    }

    (-1)

} 





fn build_table(input: String) -> Vec<i32>{

    let chars : Vec<char> = input.chars().collect();
    let m = chars.len();

    let mut table:Vec<i32> = Vec::with_capacity(m);
    table.resize(m, 0);
    table[0] = -1;
    table[1] = 0;


    let mut j = 0;
    let mut pos = 2;

    while pos < m {
        if chars[pos-1] == chars[j] {
            j += 1;
            table[pos] = j as i32;
            pos += 1;
            
        } else if j > 0 {
            j = table[j] as usize; //not safe from overflows, should fix that at some point.
        } else {
            table[pos] = 0;
            pos += 1;
        }
        
    }

    table


}
