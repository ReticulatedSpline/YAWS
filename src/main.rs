use std::io;
use std::fs;

fn main() {
	let mut correct_chars = ['\0'; 5];
	let mut known_chars = String::new();
	let mut current_guess = ['H', 'E', 'L', 'L', 'O'];
	let mut dict_vec:Vec<[char; 5]> = read_dict();

	loop {
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
		print_status(correct_chars, &known_chars);
		update_guess(&dict_vec);
	}
	println!("Nice!");
}

// Prune the dictionary vector and choose the next guess.
fn update_guess(dict_vec: &Vec<[char; 5]>) {
	dict_vec;
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
			temp[index] = character;
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

	println!("Which characters are in the correct position? (0..4)");
	io::stdin()
		.read_line(&mut correct_chars_input)
		.unwrap();

	println!("Which characters are in the word but the wrong position? (0..4)");
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

// Print a status message to the console.
fn print_status(correct_chars:[char; 5], known_chars: &str) {
	println!("Known characters: ");
	for character in correct_chars {
		if character == '\0' {
			print!("_");
		}
		else {
			print!("{}", character);
		}
	}
	print!("\n");

	print!("Also includes characters: ");
	for character in known_chars.chars() {
		if character != '\0' {
			print!("{} ", character);
		}
	}
	print!("\n");
}