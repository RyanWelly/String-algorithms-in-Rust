use std::env;
use std::fs;


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
        
    }
}


fn build_table(input: String) -> Vec<i32>{

    let mut table = vec![0; 5];
    table[0] = -1;
    table[0] = 0;

    let chars : Vec<char> = input.chars().collect();
    let mut m = chars.len();

    let mut j = 0;
    let mut pos = 2;

    while(pos < m) {

        
    }






    table
}
