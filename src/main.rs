use std::io;
use std::io::{Write};
use std::fs;
use std::process;

// TODO: Pack state into a struct
// TODO: Split evaluate_guess into prompt_user and evaluate_guess
// TODO: Functionalize dictionary length check and program exit
fn main() {
	let mut correct_chars_correct_position = ['\0'; 5];
	let mut correct_char_wrong_position: Vec<(usize, char)> = Vec::new();
	let mut wrong_chars_wrong_position: Vec<char> = Vec::new();
	let mut current_guess = ['S', 'L', 'I', 'C', 'E'];
	let mut dictionary = read_dictionary_from_file();

	print_starting_status();
	loop {
		print_status(correct_chars_correct_position, correct_char_wrong_position, current_guess);
		print!("Solved? (y/n) > ");
		let _ = io::stdout().flush();
		let mut answer = String::new();
		
		io::stdin()
			.read_line(&mut answer)
			.unwrap();
		
		match answer.as_str().trim() {
			"y" => break,
			"n" => evaluate_guess(&mut correct_chars_correct_position, &mut correct_char_wrong_position, &mut wrong_chars_wrong_position, &mut current_guess),
			_ => println!("Please respond with y/n."),
		}
		//std::process::Command::new("clear").status().unwrap();
		current_guess = update_guess(&mut dictionary, correct_chars_correct_position, correct_char_wrong_position, wrong_chars_wrong_position);
	}
	println!("Nice!");
}

// Queries the user for the outcome of the guess.
fn evaluate_guess(
	correct_chars_correct_position: &mut [char],
	correct_char_wrong_position: &mut Vec<(usize, char)>,
	wrong_chars_wrong_position: &mut Vec<char>,
	current_guess: &mut [char]) {

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
		correct_chars_correct_position[integer - 1] = current_guess[integer - 1];
		incorrect_chars[integer - 1] = false;
	}

	for character in known_chars_input.chars() {
		if character < '1' || character > '5' {
			continue;
		} else {
			let integer = character.to_digit(10).unwrap() as usize;
			incorrect_chars[integer - 1] = false;
			correct_char_wrong_position.push((integer - 1, current_guess[integer - 1]));
		}
	}

	for (index, boolean) in incorrect_chars.iter().enumerate() {
		if boolean == &true {
			wrong_chars_wrong_position.push(current_guess[index]);
		}
	}
}

// Prune the dictionary vector and choose the next guess.
fn update_guess(
	dictionary: &mut Vec<[char; 5]>,
	correct_chars_correct_position: [char;5],
	correct_char_wrong_position: Vec<(usize, char)>,
	wrong_chars: Vec<char>) -> [char; 5] {
	
	// right letter right position
	// prune words not containing known letters in the correct position
	let mut starting_dictionary_size = dictionary.len();
	for (index, character) in correct_chars_correct_position.iter().enumerate() {
		// skip unknowns, represented with \0
		if !character.is_ascii_alphabetic() {
			continue;
		} else {
			dictionary.retain(|&x| x[index] == *character);
		}
	}

	println!("Pruned {} word(s) using correct characters in the correct position.",
		starting_dictionary_size - dictionary.len());

	if dictionary.len() <= 0 {
		println!("No possibilities remain.");
		process::exit(0);
	}

	// right letter wrong position
	// prune words not containing wrongly positioned but known characters
	starting_dictionary_size = dictionary.len();
	for (index, character) in correct_char_wrong_position {
		if !character.is_ascii_alphabetic() {
			continue;
		}
		dictionary.retain(|&x| x.contains(&character));
		dictionary.retain(|&x| x[index] != character);
	}

	println!("Pruned {} word(s) using correct characters in the wrong position.",
		starting_dictionary_size - dictionary.len());

	// wrong letter wrong position
	// prune wrong characters
	starting_dictionary_size = dictionary.len();
	for character in wrong_chars {
		dictionary.retain(|&x| !x.contains(&character));
	}

	println!("Pruned {} word(s) using known incorrect characters.",
		starting_dictionary_size - dictionary.len());
	
	if dictionary.len() <= 0 {
		println!("No possibilities remain.");
		process::exit(0);
	}
	
	return dictionary.remove(0);
}

// Print a status message to the console.
fn print_status(correct_chars:[char; 5], known_chars: Vec<(usize, char)>, current_guess:[char; 5]) {
	print!("    Status: ");
	for character in correct_chars {
		if character == '\0' {
			print!("_");
		}
		else {
			print!("{}", character);
		}
	}
	print!(" | ");

	for tuple in known_chars {
		let character = tuple.1;
		if character != '\0' {
			print!("{} ", character);
		}
	}
	println!("\nNext guess: {:?}", current_guess);
	println!("            12345");
}

fn print_starting_status() {
	std::process::Command::new("clear").status().unwrap();
	println!("Pruned -- word(s).");
}

// Reads the dictionary file and returns the contents as a vector.
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