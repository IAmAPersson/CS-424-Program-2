/*
	Phil Lane
	03/14/2020
	CS-424-02
	Mary Allen

	Programming Assignment 2 (Rust)
	
	This program takes in a file containing baseball player information, and outputs to the screen calculated data about each player, such as batting average and on base percentage.

	I have opted to program in a traditional structured paradigm. Hence, the paradigm used in this program is very similar to what you would see in a C program.
	Classes are unused-- I stick to global functions and structs.
*/

#![allow(non_snake_case)]
mod funcs;
use funcs::GetPath;
use funcs::ReadInFile;
use funcs::ParseInfo;
use funcs::Calculate;
use funcs::FormatData;
use funcs::PlayerSort;

//Function: main, entrypoint
fn main() {
	//Get the path of the file to read from
	let path = GetPath();

	//Read all the data from the file
	let res = ReadInFile(path);
	
	//If there has been an error in reading from the file, exit program
	let data = match res {
		Ok(data) => data,
		Err(err) => {
			panic!("Problem opening the file: {:?}", err);
		},
	};
	
	//Parse file and sort info
	let (mut batters, badlines) = ParseInfo(data);
	batters = PlayerSort(batters);
	
	//Calculate data about batters
	let calcData = Calculate(batters);
	
	//Output calculated data to the screen
	FormatData(calcData, badlines);
}
