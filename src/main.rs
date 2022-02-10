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

	print_starting_status();

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

	print!("Are any new letters in the right position (1..5)? > ");
	let mut correct_chars_input = String::new();
	let _ = io::stdout().flush();
	io::stdin()
	.read_line(&mut correct_chars_input)
	.unwrap();
	
	print!("Are any new letters in the wrong position (1..5)? > ");
	let mut known_chars_input = String::new();
	let _ = io::stdout().flush();
	io::stdin()
	.read_line(&mut known_chars_input)
	.unwrap();
	
	let mut incorrect_chars = [true; 5];
	for character in correct_chars_input.chars() {
		if character < '1' || character > '5' {
			continue;
		}
		let integer = character.to_digit(10).unwrap() as usize;
		game_state.correct_chars[integer - 1] = game_state.current_guess[integer - 1];
		incorrect_chars[integer - 1] = false;
	}

	for character in known_chars_input.chars() {
		if character < '1' || character > '5' {
			continue;
		} else {
			let integer = character.to_digit(10).unwrap() as usize;
			incorrect_chars[integer - 1] = false;
			game_state.misplaced_chars.push((integer - 1, game_state.current_guess[integer - 1]));
		}
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
		game_state.misplaced_chars.retain(|&(_, mischar)| mischar != *correct_char);
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

	let remaining = game_state.dictionary.len();
	print!("{} possibilities remain", remaining);
	if remaining <= 5 {
		print!(": {}\n", dictionary_to_string(&game_state.dictionary));
	} else {
		print!(".\n");
	}
	game_state.current_guess = game_state.dictionary.remove(0);
}

fn print_status(game_state: &GameState) {
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

	let guess_string = String::from_iter(game_state.current_guess.into_iter());
	println!("\nNext guess: {:}", guess_string);
	println!("            12345");
}

fn print_starting_status() {
	std::process::Command::new("clear").status().unwrap();
	println!("-- possibilities remain.");
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
	let len = dictionary.len();
	let mut idx = 1;
	for array in dictionary {
		let newstr = String::from_iter(array);
		string.push_str(&newstr);
		if len != idx {
			string.push_str(", ");
		}
		idx += 1;
	}
	return string;
}