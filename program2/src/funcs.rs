/*
	Phil Lane
	03/14/2020
	CS-424-02
	Mary Allen

	Programming Assignment 2 (Rust)
*/

#![allow(non_snake_case)]
mod types;
use types::BatterInfo;
use types::CalculatedBatterInfo;
use std::io::stdin;
use std::fs;
use std::io::Result;
use std::vec::Vec;
use std::char;

//Function: Get the path of file to read from
pub fn GetPath() -> String {
	//Prompt the user
	println!("{}", "Welcome to the player statistics calculator test program! I am going to\n\
		read players from an input data file. You will tell me the name of your\n\
		input file. I will store all of the players in a list, compute each player's\n\
		averages, and then write the resulting team report to your output file!\n");
		
	println!("Enter the name of your input file:");
	
	//Create a String and read in the path of the text file
	let mut path = String::new();
	stdin().read_line(&mut path).unwrap();
	path.pop();
	path.pop();
	
	//Return the path
	path
}

//Function: Read in a file
pub fn ReadInFile(path: String) -> Result<String> {
	//Read in the file data
	let filedata = fs::read_to_string(path)?;
	
	//return
	Ok(filedata)
}

//Function: Parse the input file into a vector of BatterInfo (defined in types.rs) and a Vec<String> of all the error messages
pub fn ParseInfo(data: String) -> (Vec<BatterInfo>, Vec<String>) {
	//Delcare two vectors, a Vec<BatterInfo> to hold the parsed data for each batter, and a Vec<String> for all the error messages
	let mut batters = Vec::<BatterInfo>::new();
	let mut invalidString = Vec::<String>::new();
	
	//Comb out the \r's and split on \n, getting each line separately in an array
	let filtdata = data.replace("\r", "");
	let lines: Vec<String> = filtdata.split("\n").map(|s| s.to_string()).collect();
	
	//create a count variable for counting line numbers
	let mut cnt: u32 = 0;
	
	//Loop: loop through each line of the input file and parse
	'PrimaryLoop: for i in lines {
		//increment the count variable
		cnt = cnt + 1;
		
		//Declare two variables: one a vector of Strings for each token in the line, and one that's simply the input line split on spaces
		let mut tokens = Vec::<String>::new();
		let spaceSeparatedValues: Vec<String> = i.split(" ").map(|s| s.to_string()).collect();
		
		//Iterate through spaceSeparatedValues, and if the element is not empty, copy over to tokens
		//This ensures that the extra spaces are ignored
		for j in spaceSeparatedValues {
			if j != "" {
				tokens.push(j);
			}
		}
		
		//If there are not 10 tokens in the line, then append the error to the Vec<String> to be returned, and continue from the top with the next line
		if tokens.len() != 10 {
			invalidString.push(format!("Invalid line entered (line {})-- incorrect number of parameters.", char::from_digit(cnt, 10).unwrap()));
			continue;
		}

		//Create the batter struct
		let mut batter = BatterInfo {
			firstName: tokens[0].clone(),
			lastName: tokens[1].clone(),
			plateAppearances: 0,
			atBats: 0,
			singles: 0,
			doubles: 0,
			triples: 0,
			homeRuns: 0,
			walks: 0,
			hitByPitch: 0
		};
		
		//Individually convert and test for each field.
		//I tried to condense this like I did in my Go program, to no avail. Rust was just not working with me on this one.
		//I spent hours trying different approaches, and they all failed. This awful code was my last resort. Believe me, I tried Cells, I tried &mut, I tried everything.
		//This is why I would make a bad software engineer (or why Rust is painful to work in--you decide).
		batter.plateAppearances = match tokens[2].parse() {
			Ok(num) => num,
			Err(_err) => {
				invalidString.push(format!("Invalid line entered (line {})-- illegal type of parameter.", char::from_digit(cnt, 10).unwrap()));
				continue 'PrimaryLoop;
			},
		};
		
		batter.atBats = match tokens[3].parse() {
			Ok(num) => num,
			Err(_err) => {
				invalidString.push(format!("Invalid line entered (line {})-- illegal type of parameter.", char::from_digit(cnt, 10).unwrap()));
				continue 'PrimaryLoop;
			},
		};
		
		batter.singles = match tokens[4].parse() {
			Ok(num) => num,
			Err(_err) => {
				invalidString.push(format!("Invalid line entered (line {})-- illegal type of parameter.", char::from_digit(cnt, 10).unwrap()));
				continue 'PrimaryLoop;
			},
		};
		
		batter.doubles = match tokens[5].parse() {
			Ok(num) => num,
			Err(_err) => {
				invalidString.push(format!("Invalid line entered (line {})-- illegal type of parameter.", char::from_digit(cnt, 10).unwrap()));
				continue 'PrimaryLoop;
			},
		};
		
		batter.triples = match tokens[6].parse() {
			Ok(num) => num,
			Err(_err) => {
				invalidString.push(format!("Invalid line entered (line {})-- illegal type of parameter.", char::from_digit(cnt, 10).unwrap()));
				continue 'PrimaryLoop;
			},
		};
		
		batter.homeRuns = match tokens[7].parse() {
			Ok(num) => num,
			Err(_err) => {
				invalidString.push(format!("Invalid line entered (line {})-- illegal type of parameter.", char::from_digit(cnt, 10).unwrap()));
				continue 'PrimaryLoop;
			},
		};
		
		batter.walks = match tokens[8].parse() {
			Ok(num) => num,
			Err(_err) => {
				invalidString.push(format!("Invalid line entered (line {})-- illegal type of parameter.", char::from_digit(cnt, 10).unwrap()));
				continue 'PrimaryLoop;
			},
		};
		
		batter.hitByPitch = match tokens[9].parse() {
			Ok(num) => num,
			Err(_err) => {
				invalidString.push(format!("Invalid line entered (line {})-- illegal type of parameter.", char::from_digit(cnt, 10).unwrap()));
				continue 'PrimaryLoop;
			},
		};
		
		//Append the temporary BatterInfo object to the BatterInfo vector
		batters.push(batter);
	}
	
	//Return all the batters and the error lines
	(batters, invalidString)
}

//Function: Sort the players
pub fn PlayerSort(mut batters: Vec<BatterInfo>) -> Vec<BatterInfo> {
	//Sort the players
	batters.sort_by(|i, j| {
		//Sort by last name first, and if those are the same, then sort by first name
		if i.lastName != j.lastName {
			return i.lastName.cmp(&j.lastName)
		} else {
			return i.firstName.cmp(&j.firstName);
		}
	});
	
	//Return the sorted array
	batters
}

//Function: Calculate all the data about the batters to a CalculatedBatterInfo vector
pub fn Calculate(batters: Vec<BatterInfo>) -> Vec<CalculatedBatterInfo> {
	//Make a vector of CalculatedBatterInfo
	let mut newbatters = Vec::<CalculatedBatterInfo>::new();
	
	//Iterate through the batters vector
	for i in batters {
		let batter = CalculatedBatterInfo {
			//Copy over the first and last name
			firstName: i.firstName,
			lastName: i.lastName,
			//Calculate the batting average
			average: (i.singles + i.doubles + i.triples + i.homeRuns) as f64 / i.atBats as f64,
			//Calculate the slugging average
			slugging: (i.singles + 2 * i.doubles + 3 * i.triples + 4 * i.homeRuns) as f64 / i.atBats as f64,
			//Calculate the on base percent
			onBase: (i.singles + i.doubles + i.triples + i.homeRuns + i.walks + i.hitByPitch) as f64 / i.plateAppearances as f64,
		};
		
		//Push the CalculatedBatterInfo object to the newbatters vector.
		newbatters.push(batter);
	}
	
	//Return the CalculatedBatterInfo vector
	newbatters
}

//Function: Get the team batting average
pub fn Average(batters: &Vec<CalculatedBatterInfo>) -> f64 {
	//Create a float64 for a running total, initialized to 0.0
	let mut runningTotal = 0 as f64;
	
	//Iterate through the batters vector
	for i in batters {
		//Add to the running total
		runningTotal = runningTotal + i.average;
	}
	
	//Return the running total divided by the length of the inputted vector (i.e. the average)
	runningTotal / batters.len() as f64
}

//Function: Print the formatted data to the screen
pub fn FormatData(batters: Vec<CalculatedBatterInfo>, errorlines: Vec<String>) {
	//Print the players found and the team average
	println!("\nBASEBALL TEAM REPORT --- {:?} PLAYERS FOUND IN FILE", batters.len());
	println!("OVERALL BATTING AVERAGE is {:.3}\n", Average(&batters));
	
	//Print the top of the chart
	println!("    PLAYER NAME      :    AVERAGE  SLUGGING   ONBASE%");
	println!("-----------------------------------------------------");
	
	//Iterate through the batters vector and print the information, formatted to the screen
	for i in batters {
		println!("{:20} :      {:.3}     {:.3}     {:.3}", format!("{}, {}", i.lastName, i.firstName), i.average, i.slugging, i.onBase);
	}
	
	//Print the number of error lines
	println!("\n----- {:?} ERROR LINES FOUND IN INPUT DATA -----\n", errorlines.len());
	
	//Print the error lines
	for i in errorlines {
		println!("{}", i);
	}
}
