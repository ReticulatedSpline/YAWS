use std::io;
use std::io::{Write};
use std::fs;

struct GameState {
	dictionary : Vec<[char; 5]>,
	correct_chars: [char; 5],
	misplaced_chars: Vec<(usize, char)>,
	incorrect_chars: Vec<char>,
	current_guess: [char; 5]
}

fn main() {

	let mut game_state = GameState {
		dictionary: read_dictionary_from_file(),
		correct_chars: ['\0'; 5],
		misplaced_chars: Vec::new(),
		incorrect_chars: Vec::new(),
		current_guess: ['S', 'L', 'I', 'C', 'E']
	};

	loop {
		print_status(&game_state);
		print!("Solved? (y/n) > ");
		let _ = io::stdout().flush();
		let mut answer = String::new();
		
		io::stdin()
			.read_line(&mut answer)
			.unwrap();
		
		match answer.as_str().trim() {
			"y" => break,
			"n" => evaluate_guess(&mut game_state),
			 _  => println!("Please respond with y/n."),
		}
		
		std::process::Command::new("clear").status().unwrap();
		update_guess(&mut game_state);
	}
	println!("Nice!");
}

fn evaluate_guess(game_state: &mut GameState) {

	print!("Are any new letters in the right position (0..4)? > ");
	let mut correct_chars_input = String::new();
	let _ = io::stdout().flush();
	io::stdin()
	.read_line(&mut correct_chars_input)
	.unwrap();
	
	print!("Are any new letters in the wrong position (0..4)? > ");
	let mut known_chars_input = String::new();
	let _ = io::stdout().flush();
	io::stdin()
		.read_line(&mut known_chars_input)
		.unwrap();
	
	let mut incorrect_chars = [true; 5];
	for input in correct_chars_input.chars() {
		if input < '0' || input > '4' {
			continue;
		}
		let position = input.to_digit(10).unwrap() as usize;
		game_state.correct_chars[position] = game_state.current_guess[position];
		incorrect_chars[position] = false;
	}

	for input in known_chars_input.chars() {
		if input < '0' || input > '4' {
			continue;
		}
		let position = input.to_digit(10).unwrap() as usize;
		incorrect_chars[position] = false;
		game_state.misplaced_chars.push((position, game_state.current_guess[position]));

	}

	for (index, boolean) in incorrect_chars.iter().enumerate() {
		let character = game_state.current_guess[index];
		// don't push anything in correct chars as there could be duplicates
		if *boolean && !game_state.correct_chars.contains(&character) {
			game_state.incorrect_chars.push(character);
		}
	}
}

fn update_guess(game_state: &mut GameState) {
	
	// prune dictionary based on correct letters
	for (index, correct_char) in game_state.correct_chars.iter().enumerate() {
		if !correct_char.is_ascii_alphabetic() {
			continue;
		}
		// remove discovered characters from misplaced_chars
		//game_state.misplaced_chars.retain(|&(_, mischar)| mischar != *correct_char);
		game_state.dictionary.retain(|&x| x[index] == *correct_char);
	}

	// prune dictionary based on misplaced letters
	for (index, character) in &game_state.misplaced_chars {
		if !character.is_ascii_alphabetic() {
			continue;
		}
		game_state.dictionary.retain(|&x| x.contains(&character));
		game_state.dictionary.retain(|&x| x[*index] != *character);
	}

	// prune dictionary based on incorrect characters
	for character in &game_state.incorrect_chars {
		game_state.dictionary.retain(|&x| !x.contains(&character));
	}

	if let Some(top) = game_state.dictionary.pop() {
		game_state.current_guess = top;
	} else {
		println!("No possibilities remain.");
		std::process::exit(1);
	}
}

fn print_status(game_state: &GameState) {
	let remaining = game_state.dictionary.len();
	let guess_string = String::from_iter(game_state.current_guess.into_iter());
	print!("{} choice(s) remain", remaining);
	if remaining <= 5 {
		print!(": {}\n", dictionary_to_string(&game_state.dictionary));
	} else {
		print!(".\n");
	}

	print!("    Status: ");
	for character in &game_state.correct_chars {
		if character == &'\0' {
			print!("_");
		}
		else {
			print!("{}", character);
		}
	}
	print!(" | ");

	for (_, character) in &game_state.misplaced_chars {
		if character.is_ascii_alphabetic() {
			print!("{} ", character);
		}
	}

	println!("\nNext guess: {:}", guess_string);
	println!("            01234");
}

fn read_dictionary_from_file() -> Vec<[char; 5]> {
	let dict_string = fs::read_to_string("./src/wordle_dict.dat")
		.expect("Could not read dictionary file.");
	let mut dict_vec = Vec::new();
	let mut index = 0;
	let mut temp: [char; 5] = ['\0'; 5];

	for character in dict_string.chars() {
		if index < 5 && character != '\n' {
			temp[index] = character.to_ascii_uppercase();
			index += 1;
		} else {
			dict_vec.push(temp);
			temp = ['\0'; 5];
			index = 0;
		}
	}
	return dict_vec;
}

fn dictionary_to_string(dictionary:&Vec<[char ; 5]>) -> String {
	let mut string = String::new();
	for array in dictionary {
		let newstr = String::from_iter(array);
		string.push_str(&newstr);
		string.push_str(" ");
	}
	return string;
}