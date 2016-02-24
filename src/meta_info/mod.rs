extern crate bencoding_rs;

use self::bencoding_rs::BencodingResult;

struct MetaInfo {
	announce: String,
	info: ContentInfo
}

struct ContentInfo {
	piece_length: u32,
	pieces: String,
	fileInfo: FileInfo
}

enum FileInfo {
	SingleFileInfo,
	MultiFileInfo
}

struct SingleFileInfo {
	length:u32,
	name: String
}

struct MultiFileInfo {
	files: Vec<File>
}

struct File {
	path: Vec<String>,
	name:String
}