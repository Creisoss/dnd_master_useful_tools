extern crate rand;

use std::io;
use rand::Rng;

struct DiceRoll {
    number_of_rolls: u32,
    dice_size: u32,
    operator: char,
    operation_number: u32,
}

fn main(){
    loop{
        clear();
        println!("Create a new dice");
        let x = DiceRoll::new_dice_roll();
        roll_dice(x);
        println!("do you want to stop(y, leave blank to repeat)");
        let op = input_char();
        if op == 'y' { break; }
    }
}

impl DiceRoll{
    fn new_dice_roll() -> DiceRoll{
        
        println!("How many times will you roll your dice? ");
        let number_of_rolls = input_number();
        
        println!("Hom many sides your dice has? ");
        let dice_size = input_number();
        
        println!("Will you use any operation? (+, -, *, /, leave blank if not) ");
        let operator = input_char();
        
        let mut operation_number: u32 = 0;
        if operator == '+' || operator == '-' || operator == '/' || operator == '*'{
            println!("What number will you use in your operation? ");
            operation_number = input_number();
            if operator == '/' && operation_number == 0 {
                panic!("cannot divide by zero");
            }
        }
        
        let roll: DiceRoll = DiceRoll { 
            number_of_rolls, 
            dice_size, 
            operator, 
            operation_number};
        roll
    }
}
fn clear(){
    print!("\x1B[2J\x1B[1;1H");
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
        }else {
            x = rng.gen_range(1, roll.dice_size+1);
        }
        match roll.operator {
            '+' => x + roll.operation_number,
            '-' => x - roll.operation_number,
            '*' => x * roll.operation_number,
            '/' => x / roll.operation_number,
             _ => 1+1,
        };

        total = total + x;
        print!("{x}");
    }
    println!("}}");
    println!("{}", total);
}

fn input_char() -> char{
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("failed to read from stdin");
    let trimmed = input_text.trim();
    match trimmed.parse::<char>() {
        Ok(i) =>   i,
        Err(..) => '\0',
    }
}

fn input_number() -> u32{
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("failed to read from stdin");
    let trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(i) =>   i,
        Err(..) => 0,
    }
}
