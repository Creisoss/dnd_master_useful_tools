extern crate rand;

use std::io;
use rand::Rng;

struct DiceRoll {
    number_of_rolls: u32,
    dice_size: u32,
    operator: char,
    operation_number: u32,
}

impl DiceRoll{
    fn new_dice_roll(number_of_rolls: u32, dice_size: u32, operator: char, operation_number: u32) -> DiceRoll{
        let roll: DiceRoll = DiceRoll { 
            number_of_rolls, 
            dice_size, 
            operator, 
            operation_number};
        roll
    }
}


fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

fn process_input(s: &String){
    // all this variables will be used to process the string but will be deleted after the scope
    // of the function ends
    let mut index: usize = 0;
    let mut first_number_end_index: usize;
    let mut number_of_rolls: u32 = 0;
    let mut second_number_first_index: usize = 0;
    let mut second_number_last_index: usize;
    let mut valuable_index: usize = 0;
    let mut read_second_value: bool = false;
    let mut dice_size: u32 = 0;
    let mut has_operator = false;
    let mut operator: char = '|';
    let mut operation_number_index = 0;
    let mut operation_number: u32 = 0;
    let mut last_char: char = 'a';
    for i in s.chars(){
        if index == 0 && !i.is_numeric(){
            panic!("The first value need to be a number");
        }
            
        if !i.is_numeric() && (i == 'd' || i == 'D'){
            first_number_end_index = index;
            number_of_rolls = (&s[0..first_number_end_index].parse::<u32>().unwrap()).clone();
            second_number_first_index = index + 1;
            read_second_value = true;
        }else if !i.is_numeric() && !(i == 'd' || i == 'D') && read_second_value == false{ 
            panic!("You need to input d or D to set the number of sides of the dice");
        }

        if read_second_value{
            if !i.is_numeric(){
                if index == second_number_first_index{
                    panic!("The second number need to be an int");
                }
            }else if (index == s.len() - 1) || (!i.is_numeric()) {
                if !i.is_numeric() && last_char.is_numeric(){
                    second_number_last_index = index - 1;
                }else{
                    second_number_last_index = index;
                }
                println!("{}",&s[second_number_first_index..second_number_last_index+1]);
                dice_size = (&s[second_number_first_index..second_number_last_index+1].parse::<u32>().unwrap()).clone();
                if index == s.len() - 1 && !i.is_numeric() {
                    match i {
                        '+' => operator = '+',
                        '-' => operator = '-',
                        '/' => operator = '/',
                        '*' => operator = '*',
                        _ => panic!("Didn't recognize this symbol"),
                    }; 
                    has_operator = true;
                }
            }
        }
        
        if has_operator && i.is_numeric() {
            operation_number_index = index;
        }else if has_operator && !i.is_numeric(){
            if index == s.len() - 1{
                panic!("The third value need to be a number");
            }else{
                operation_number = (&s[operation_number_index..index - 1].parse::<u32>().unwrap()).clone();
            }
        }
        last_char = i;
        index = index + 1;
    }
    println!("{number_of_rolls}, {dice_size}, {operator}, {operation_number}");
    let roll = DiceRoll::new_dice_roll(number_of_rolls, dice_size, operator, operation_number);
    roll_dice(roll);
}

fn roll_dice(roll: DiceRoll){
    let mut rng = rand::thread_rng();
    let mut total: u32 = 0;
    print!("{{");
    let mut x;
    for i in 0..roll.number_of_rolls{
        if (i != roll.number_of_rolls) && (i != 0) {
            print!(", ");
        }
        if roll.dice_size == 1 {
            x = 1;
            total = total + x;
        }else {
            x = rng.gen_range(1, roll.dice_size);
            total = total + x;
        }
        print!("{x}");
    }
    println!("}}");
    println!("{}", total);
}

fn main() {
    let mut dice_roll: String = String::new(); 
    io::stdin().read_line(&mut dice_roll).expect("Failed to read line");
    remove_whitespace(&mut dice_roll);
    process_input(&dice_roll);
}
