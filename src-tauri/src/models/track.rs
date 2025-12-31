use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

pub fn load_tracks<P: AsRef<Path>>(path: P) -> Result<HashMap<String, String>> {
	let file = File::open(path)?;
	let reader = BufReader::new(file);
	let mut map = HashMap::new();
	for line in reader.lines() {
		let line = line?;
		let mut cols = line.splitn(2, ',');
		if let (Some(key), Some(value)) = (cols.next(), cols.next()) {
			map.insert(key.to_string(), value.to_string());
		}
	}
	Ok(map)
}