use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read<P: AsRef<Path>>(path:P) -> Result<String,std::io::Error>{
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
