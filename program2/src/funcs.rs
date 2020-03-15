/*
	Phil Lane
	03/14/2020
	CS-424-02
	Mary Allen

	Programming Assignment 2 (Rust)
*/

mod types;
use types::BatterInfo;
use types::CalculatedBatterInfo;
use std::io::stdin;
use std::fs::File;
use std::io::Result;
use std::vec::Vec;
use std::char;
use std::panic;

//Function: Get the path of file to read from
pub fn GetPath() -> String {
	//Prompt the user
	println!("{}", "Welcome to the player statistics calculator test program! I am going to\n\
		read players from an input data file. You will tell me the name of your\n\
		input file. I will store all of the players in a list, compute each player's\n\
		averages, and then write the resulting team report to your output file!\n");
		
	print!("Enter the name of your input file: ");
	
	//Create a String and read in the path of the text file
	let mut path = String::new();
	stdin().read_line(&mut path);
	
	//Return the path
	path
}

//Function: Read in a file
pub fn ReadInFile(path: String) -> Result<String> {
	//Create a file object to read in from
	let file = File::open(path)?;
	
	//Read in the file data
	let mut filedata = String::new();
	file.read_to_string(&mut filedata)?;
	
	//Close the file
	drop(file);
	
	//Cast the []byte to a String and return
	Ok(filedata)
}

//Function: Parse the input file into a slice of BatterInfo (defined in types.go) and a []String of all the error messages
pub fn ParseInfo(data: String) -> (Vec<BatterInfo>, Vec<String>) {
	//Delcare two slices, a []BatterInfo to hold the parsed data for each batter, and a []String for all the error messages
	let batters = Vec::<BatterInfo>::new();
	let invalidString = Vec::<String>::new();
	
	//Comb out the \r's and split on \n, getting each line separately in an array
	let filtdata = data.replace("\r", "");
	let lines = filtdata.split("\n").collect();
	
	//create a count variable for counting line numbers
	let mut cnt: u32 = 0;
	
	//Loop: loop through each line of the input file and parse
	'PrimaryLoop: for i in lines {
		//increment the count variable
		cnt = cnt + 1;
		
		//Declare two variables: one a slice of Strings for each token in the line, and one that's simply the input line split on spaces
		let mut tokens = Vec::<String>::new();
		let spaceSeparatedValues = i.split(" ").collect();
		
		//Iterate through spaceSeparatedValues, and if the element is not empty, copy over to tokens
		//This ensures that the extra spaces are ignored
		for j in spaceSeparatedValues {
			if j != "" {
				tokens.push(j);
			}
		}
		
		//If there are not 10 tokens in the line, then append the error to the []String to be returned, and continue from the top with the next line
		if tokens.len() != 10 {
			invalidString.push(format!("Invalid line entered (line {})-- incorrect number of parameters.", char::from_digit(cnt, 10).unwrap()));
			continue;
		}
		
		let batter = panic::catch_unwind(|| { BatterInfo {
			firstName: tokens[0],
			lastName: tokens[1],
			plateAppearances: tokens[2].parse().unwrap(),
			atBats: tokens[3].parse().unwrap(),
			singles: tokens[4].parse().unwrap(),
			doubles: tokens[5].parse().unwrap(),
			triples: tokens[6].parse().unwrap(),
			homeRuns: tokens[7].parse().unwrap(),
			walks: tokens[8].parse().unwrap(),
			hitByPitch: tokens[9].parse().unwrap(),
		}});
		
		let resbatter = match batter {
			Ok(res) => res,
			Err(err) => {
				invalidString.push(format!("Invalid line entered (line {})-- illegal type of parameter.", char::from_digit(cnt, 10).unwrap()));
				continue 'PrimaryLoop;
			}
		};
		
		//Append the temporary BatterInfo object to the BatterInfo slice
		batters.push(resbatter);
	}
	
	//Return all the batters and the error lines
	return (batters, invalidString);
}

//Function: Sort the players
pub fn PlayerSort(batters: Vec<BatterInfo>) -> Vec<BatterInfo> {
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
	return batters;
}

//Function: Calculate all the data about the batters to a CalculatedBatterInfo slice
pub fn Calculate(batters: Vec<BatterInfo>) -> Vec<CalculatedBatterInfo> {
	//Make a slice of CalculatedBatterInfo with as many elements as there are in the batters slice
	let mut newbatters = Vec::<CalculatedBatterInfo>::new();
	
	//create a count variable for counting line numbers
	let mut cnt: usize = 0;
	
	//Iterate through the batters slice
	for i in batters {
		cnt = cnt + 1;
		
		//Copy over the first and last name
		newbatters[cnt].firstName = i.firstName;
		newbatters[cnt].lastName = i.lastName;
		//Calculate the batting average
		newbatters[cnt].average = (i.singles + i.doubles + i.triples + i.homeRuns) as f64 / i.atBats as f64;
		//Calculate the slugging average
		newbatters[cnt].slugging = (i.singles + 2 * i.doubles + 3 * i.triples + 4 * i.homeRuns) as f64 / i.atBats as f64;
		//Calculate the on base percent
		newbatters[cnt].onBase = (i.singles + i.doubles + i.triples + i.homeRuns + i.walks + i.hitByPitch) as f64 / i.plateAppearances as f64;
	}
	
	//Return the CalculatedBatterInfo slice
	return newbatters;
}

//Function: Get the team batting average
pub fn Average(batters: Vec<CalculatedBatterInfo>) -> f64 {
	//Create a float64 for a running total, initialized to 0.0
	let mut runningTotal = 0 as f64;
	
	//Iterate through the batters slice
	for i in batters {
		//Add to the running total
		runningTotal = runningTotal + i.average;
	}
	
	//Return the running total divided by the length of the inputted slice (i.e. the average)
	return runningTotal / batters.len() as f64;
}

//Function: Print the formatted data to the screen
pub fn FormatData(batters: Vec<CalculatedBatterInfo>, errorlines: Vec<String>) {
	//Print the players found and the team average
	println!("\nBASEBALL TEAM REPORT --- {:?} PLAYERS FOUND IN FILE", batters.len());
	println!("OVERALL BATTING AVERAGE is {:.3}\n", Average(batters));
	
	//Print the top of the chart
	println!("    PLAYER NAME      :    AVERAGE  SLUGGING   ONBASE%");
	println!("-----------------------------------------------------");
	
	//Iterate through the batters slice and print the information, formatted to the screen
	for i in batters {
		println!("{:20} :      {:.3}     {:.3}     {:.3}", format!("{}, {}", &i.lastName, &i.firstName), &i.average, &i.slugging, &i.onBase);
	}
	
	//Print the number of error lines
	println!("\n----- {:?} ERROR LINES FOUND IN INPUT DATA -----\n", errorlines.len());
	
	//Print the error lines
	for i in errorlines {
		println!("{}", i);
	}
}
