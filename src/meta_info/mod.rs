extern crate bencoding_rs;

use self::bencoding_rs::{BencodingResult, decoder};
use std::collections::HashMap;
use std::default::Default;


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
	name:String
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

	retrieve_url(&mut meta_info, &bencoded_map);
	retrieve_info_raw(&mut meta_info, &bencoded_map);

	return Err("error")
}

fn retrieve_info_raw(meta_info:&mut MetaInfo, bencoded_map: &HashMap<String, BencodingResult>) {
	 	let url_option: Option<&BencodingResult> = bencoded_map.get("announce");

}

fn retrieve_url(meta_info:&mut MetaInfo, bencoded_map: &HashMap<String, BencodingResult>) {
	let url_option: Option<&BencodingResult> = bencoded_map.get("announce");
	let url_result = match url_option {
		Some(&BencodingResult::ByteString(ref url_vect)) => String::from_utf8(url_vect.clone()),
		_ => panic!("Not a valid announce url")
	};

	if let Ok(url) = url_result {
		meta_info.tracker_url = url;
	} else {
		panic!("url is not valid");
	}
}
