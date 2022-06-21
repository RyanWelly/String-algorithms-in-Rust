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
