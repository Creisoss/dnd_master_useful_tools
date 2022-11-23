extern crate dndtools;

use dndtools::input;
use dndtools::dice;


fn main(){
    loop{
        println!("Choose an option:\n1-Roll a dice\n0-Exit");
        let op = input::input_char();
        match op {
            '1' => { 
                    println!("Create a new dice");
                    let dice = dice::DiceRoll::new_dice_roll();
                    clear();
                    dice.roll_dice();
                    },
            '0' => {
                    clear();
                    println!("Bye!!");
                    break;
                    },
            _ => continue,
        };    
    }
}


fn clear(){
    print!("\x1B[2J\x1B[1;1H");
}
