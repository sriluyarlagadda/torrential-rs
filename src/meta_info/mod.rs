extern crate bencoding_rs;

use self::bencoding_rs::{BencodingResult, decoder};
use std::collections::HashMap;
use std::default::Default;
use std::clone::Clone;
use std::error::Error;
use std::string::FromUtf8Error;


pub struct MetaInfo {
	tracker_url: String,
	info_raw: String,
	piece_length: u32,
	pieces_sha1: Vec<u8>,
	file_info:FileInfo
}

impl Default for MetaInfo {
	fn default() -> Self {
		let file_info:FileInfo = FileInfo::SingleFileInfo{length:0, name: String::new()};
		let mut meta_info:MetaInfo = MetaInfo { tracker_url:String::new(), 
				info_raw:String::new(), piece_length:0, pieces_sha1:Vec::new(), file_info:file_info};
		
		return meta_info;
	}
}

enum FileInfo {
	SingleFileInfo {length:u32, name:String},
	MultiFileInfo {files:Vec<File>}
}

#[derive(Default)]
struct File {
	path: Vec<String>,
	name:String,
	length:u32
}

pub fn to_meta_info(input: Vec<u8>) -> Result<MetaInfo, &'static str> {
	let torrent_result:Result<BencodingResult, &str> = decoder::decode(input);

	if let Ok(bencoded_torrent) = torrent_result {
		return populate_meta_info(bencoded_torrent)
	} else {
		return Err(torrent_result.err().unwrap())
	}
}

fn populate_meta_info(bencoded_result: BencodingResult) -> Result<MetaInfo, &'static str> {
	let mut meta_info:MetaInfo = Default::default();
	let mut bencoded_map:HashMap<String, BencodingResult>;

	bencoded_map = match bencoded_result {
		BencodingResult::Dict(bencoded_map) => bencoded_map,
		_ => return Err("the torrent is not valid")
	};

	let tracker_url = match retrieve_url(&bencoded_map) {
		Ok(url) => url,
		Err(error) => panic!("{}", error),
	};
	meta_info.tracker_url = tracker_url;

	let info_data = try!(bencoded_map.get("info").ok_or("key 'info' does not exist"));

	let info_map:HashMap<String, BencodingResult>;
    {
        info_map =  match info_data {
            &BencodingResult::Dict(ref info_map) => info_map.clone(),
            _ => return Err("key 'info' is not a dictionary")
        };
    }

    let piece_length:u32 = match retrieve_piece_length(&info_map) {
        Ok(piece_length) => piece_length,
        Err(error) => return Err(error)
    };

    meta_info.piece_length = piece_length;    


    let pieces:Vec<u8> = match retrieve_pieces_sha1(&info_map) {
    	Ok(pieces) => pieces,
    	Err(error) => return Err(error)
    };
    meta_info.pieces_sha1 = pieces;

    for (key, _) in info_map {
    	println!("{:?}", key);
    }

	unimplemented!()
}

fn retrieve_file_info(info_map: &HashMap<String, BencodingResult>) -> Result<FileInfo, &'static str> {

	if !info_map.contains_key("name") {
		return Err("key name does not exist in the 'info' dictionary")
	}

	let name:&BencodingResult = info_map.get("name").unwrap();
	let name_as_vec:Vec<u8> = match name {
	    &BencodingResult::ByteString(ref name) => name.clone(),
	    _ => return Err("key 'name' is not a Bencoded ByteString"),
	};

	let name:String = match String::from_utf8(name_as_vec) {
		Ok(name) => name,
		Err(err) => return Err("cannot convert to string from utf 8 bytes")
	};

	unimplemented!()
}

fn retrieve_pieces_sha1(info_map: &HashMap<String, BencodingResult>) -> Result<Vec<u8>, &'static str> {
	let pieces_sha1:Vec<u8> = Vec::new();
	let pieces_sha1_option:Option<&BencodingResult> = info_map.get("pieces");
	let pieces_sha1_result:&BencodingResult =  try!(pieces_sha1_option.
									ok_or("pieces sha1 does not exist"));


	let pieces = match pieces_sha1_result {
		&BencodingResult::ByteString(ref pieces_vec) => pieces_vec.clone(),
		_ => return Err("key 'pieces' is not a Bencoded ByteString")
	};
	return Ok(pieces)
}

fn retrieve_piece_length(info_map: &HashMap<String, BencodingResult>) -> Result<u32, &'static str> {
	let piece_length: u32;
	let piece_length_option: Option<&BencodingResult> = info_map.get("piece length");
	let piece_length_result:&BencodingResult =  try!(piece_length_option.
									ok_or("piece length does not exist"));

	let piece_length = match piece_length_result {
		&BencodingResult::Int(length) => length,
		_ => return Err("key 'piece length' is not a Bencoded Integer")
	};

	return Ok(piece_length as u32)
}


fn retrieve_url(bencoded_map: &HashMap<String, BencodingResult>) -> Result<String, String> {
	let url_option: Option<&BencodingResult> = bencoded_map.get("announce");

	let url_result:&BencodingResult = 
				try!(url_option.ok_or(String::from("Tracker url does not exist")));

	let url = match url_result {
	    &BencodingResult::ByteString(ref url_vect) => String::from_utf8(url_vect.clone()),
	    _ => return Err(String::from("Invali url")),
	};

	return url.map_err(|err| err.to_string())
}