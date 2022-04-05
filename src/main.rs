use std::{collections::HashMap};
use std::io::stdin;

fn lettersum(msg: &str) {

    //HashTable instantiation
    let mut alpha_num_map: HashMap<char, i32> = HashMap::new();

    alpha_num_map.insert('a', 1);
    alpha_num_map.insert('b', 2);
    alpha_num_map.insert('c', 3);
    alpha_num_map.insert('d', 4);
    alpha_num_map.insert('e', 5);
    alpha_num_map.insert('f', 6);
    alpha_num_map.insert('g', 7);
    alpha_num_map.insert('h', 8);
    alpha_num_map.insert('i', 9);
    alpha_num_map.insert('j', 10);
    alpha_num_map.insert('k', 11);
    alpha_num_map.insert('l', 12);
    alpha_num_map.insert('m', 13);
    alpha_num_map.insert('n', 14);
    alpha_num_map.insert('o', 15);
    alpha_num_map.insert('p', 16);
    alpha_num_map.insert('q', 17);
    alpha_num_map.insert('r', 18);
    alpha_num_map.insert('s', 19);
    alpha_num_map.insert('t', 20);
    alpha_num_map.insert('u', 21);
    alpha_num_map.insert('v', 22);
    alpha_num_map.insert('w', 23);
    alpha_num_map.insert('x', 24);
    alpha_num_map.insert('y', 25);
    alpha_num_map.insert('z', 26);

    // Break String into characters
    let values = msg.chars();
    
    // Need Mutable variable for addition
    let mut sum = 0;

    for c in values 
    {
        // Need to unwrap the option to get the raw integer
        sum += alpha_num_map.get(&c).unwrap();
    }
    println!("{}: {}", msg, sum) 
}

fn main()
{
    println!("This program takes in a lowercase(a-z) string and adds each character as a key/value pair in a Hash map and returns the sum of all character values added.");
    println!("The program does not accept whitespace nor characters that are not a-z.");
    println!("(Enter'!q' to Exit)\n");
    let mut cont = true;
    // Read User input and check sum
    while cont == true
    {
        println!("Enter Text to Sum:");
        let mut input_string = String::new();
        stdin().read_line(&mut input_string)
    	    .ok()
            .expect("Failed to read line");
        let trimmed_input = input_string.trim_end();
        if trimmed_input != "!q"
        {
            lettersum(&trimmed_input);
        }
        else
        {
            cont = false;        
        }
    }

    // Prompt Check
    /*
    lettersum("");
    lettersum("a");
    lettersum("z");
    lettersum("cab");
    lettersum("excellent");
    lettersum("microspectrophotometries");
    */
}