extern crate torrential;
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Result as IO_Result;


use torrential::meta_info;

fn main() {
	let mut arguments = env::args();
	let err_msg: String = String::from("Please provide a valid torrent file path");

	let mut file_name:String = String::new();
	
	let file_name_option = arguments.nth(1);
	if let None = file_name_option {
		println!("{}", err_msg );
		return
	} 
	
	let file_name = file_name_option.unwrap();
	let mut file_content:Vec<u8> = Vec::new();

	println!("file_name: {}" ,file_name);

	let mut file_option:IO_Result<File> = File::open(file_name);
	let mut file:File;
	let file_string:Vec<char>;
	if let Ok(mut file) = file_option {
		let bytes_written:IO_Result<usize> = file.read_to_end(&mut file_content);
		if let Ok(bytes) = bytes_written {
			meta_info::to_meta_info(file_content);
		} else {
			println!("{}", bytes_written.err().unwrap());
		}
	}
}
