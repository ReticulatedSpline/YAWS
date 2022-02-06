use std::io;
use std::fs;
use std::process;

fn main() {
	let mut correct_chars = ['\0'; 5];
	let mut known_chars = String::new();
	let mut current_guess = ['S', 'L', 'I', 'C', 'E'];
	let mut dict_vec:Vec<[char; 5]> = read_dict();

	loop {
		print_status(correct_chars, &known_chars);
		println!("Next guess: {}", String::from_iter(current_guess));
		println!("            01234");
		println!("Solved? y/n");
		let mut answer = String::new();

		io::stdin()
			.read_line(&mut answer)
			.unwrap();

		match answer.as_str().trim() {
			"y" => break,
			"n" => evaluate_guess(&mut correct_chars, &mut known_chars, &mut current_guess),
			  _ => println!("Please respond with y/n."),
		}
		current_guess = update_guess(&mut dict_vec, &mut known_chars, correct_chars);
	}
	println!("Nice!");
}

// Reads the dictionary file and returns the contents as a vector.
fn read_dict() -> Vec<[char; 5]> {
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

// Queries the user for the outcome of the guess.
fn evaluate_guess(correct_chars: &mut [char], known_chars: &mut String, current_guess: &mut [char]) {
	let mut correct_chars_input = String::new();
	let mut known_chars_input = String::new();

	println!("Are any new letters in the right position (0..4)?");
	io::stdin()
		.read_line(&mut correct_chars_input)
		.unwrap();

	println!("Are any new letters in the wrong position (0..4)?");
	io::stdin()
		.read_line(&mut known_chars_input)
		.unwrap();
	
	for character in correct_chars_input.chars() {
		if character < '0' || character > '9' {
			continue;
		}
		let integer = character.to_digit(10).unwrap() as usize;
		correct_chars[integer] = current_guess[integer];
	}

	for character in known_chars_input.chars() {
		if character < '0' || character > '9' {
			break;
		} else if known_chars.contains(character) {
			continue;
		} else {
			let integer = character.to_digit(10).unwrap() as usize;
			known_chars.push(current_guess[integer]);
		}
	}
}

// Prune the dictionary vector and choose the next guess.
fn update_guess(dict_vec: &mut Vec<[char; 5]>, known_chars: &mut String, correct_chars: [char;5]) -> [char; 5] {
	//check correct letters
	let original_dict_size = dict_vec.len();
	let mut char_idx: usize = 0;
	for character in correct_chars {
		if !character.is_ascii_alphabetic() {
			char_idx += 1;
			continue;
		} else {
			dict_vec.retain(|&x| x[char_idx] == character);
		}
		char_idx += 1;
	}
	
	println!("Pruned {} word(s) with rightly positioned letters.", original_dict_size - dict_vec.len());

	if dict_vec.len() <= 0 {
		println!("No possibilities remain.");
		process::exit(0);
	}


	//check contained letters
	let original_dict_size = dict_vec.len();
	for character in known_chars.chars() {
		if !character.is_ascii_alphabetic() {
			continue;
		}
		dict_vec.retain(|&x| x.contains(&character));
	}
	println!("Pruned {} word(s) with wrongly positioned letters.", original_dict_size - dict_vec.len());
	
	if dict_vec.len() <= 0 {
		println!("No possibilities remain.");
		process::exit(0);
	}
	
	return dict_vec.remove(0);
}

// Print a status message to the console.
fn print_status(correct_chars:[char; 5], known_chars: &str) {
	println!("Status: ");
	for character in correct_chars {
		if character == '\0' {
			print!("_");
		}
		else {
			print!("{}", character);
		}
	}
	print!(" | ");

	for character in known_chars.chars() {
		if character != '\0' {
			print!("{} ", character);
		}
	}
	print!("\n\n");
}