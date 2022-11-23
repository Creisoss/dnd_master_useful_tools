use super::*;
use rand::Rng;

pub struct DiceRoll {
number_of_rolls: u32,
dice_size: u32,
operator: char,
operation_number: u32,
}
impl DiceRoll{
pub fn new_dice_roll() -> DiceRoll{

println!("How many times will you roll your dice? ");
let number_of_rolls = input::input_number();

println!("Hom many sides your dice has? ");
let dice_size = input::input_number();

println!("Will you use any operation? (+, -, *, /, leave blank if not) ");
let operator = input::input_char();

let mut operation_number: u32 = 0;
if operator == '+' || operator == '-' || operator == '/' || operator == '*'{
    println!("What number will you use in your operation? ");
    operation_number = input::input_number();
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

pub fn roll_dice(self){
    let mut rng = rand::thread_rng();
    let mut total: u32 = 0;
    let mut x;
if self.dice_size > 0{
        print!("{{");
for i in 0..self.number_of_rolls{
	
    	if (i != self.number_of_rolls) && (i != 0) {
	        print!(", ");
	    }
	
    	if self.dice_size == 1 {
	        x = 1;
	    }else {
	        x = rng.gen_range(1, self.dice_size+1);
	    }
	    match self.operator {
	        '+' => x + self.operation_number,
	        '-' => x - self.operation_number,
	        '*' => x * self.operation_number,
	        '/' => x / self.operation_number,
	        _ => 1+1,
	    };

	    total = total + x;
	    print!("{x}");
        }
        println!("}}");
}else {
total = 0;
}
    println!("{}", total);
}
}

