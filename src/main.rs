use std::process::Command;
use std::env;
use std::fs;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn check_extension(filename: &str) -> bool {
	let lowercase = filename.to_lowercase();
	let extension = &lowercase[lowercase.len()-5..];
	if extension == ".docx" {
		return true;
	}
	if extension == ".xlsx" {
		return true;
	}
	if extension == ".pptx" {
		return true;
	}
	return false;
}

fn filename_backup(filename: &str) -> String {
	let lowercase = filename.to_lowercase();
	let extension = &lowercase[lowercase.len()-5..];
	let backup = format!("{}{}{}",&filename[..filename.len()-5],"-Backup",extension);
	return backup;
}
	
fn main() {
	let args: Vec<String> = env::args().collect();
	let filename = &args[1];
	
	if ! check_extension(filename) {
		std::process::exit(1);
	}

	let temp = env::temp_dir();
	if let Some(temp_str) = temp.to_str()
	{
		let temp_string = temp_str.to_string();
		let rand_string: String = thread_rng()
			.sample_iter(&Alphanumeric)
			.take(8)
			.map(char::from)
			.collect();
		let path = temp_string + &rand_string;
		let zip = path.clone() + ".zip";
		let backup = filename_backup(filename);
			
		let _ = fs::create_dir(path.clone());
		let _ = Command::new("tar.exe").arg("-xf").arg(filename).arg("-C").arg(path.as_str()).status();
		let _ = Command::new("tar.exe").arg("-acf").arg(zip.as_str()).arg("-C").arg(path.as_str()).arg("*").status();
		let _ = fs::rename(filename, backup);
		let _ = fs::rename(zip, filename);
		let _ = fs::remove_dir_all(path);		
	}
}

