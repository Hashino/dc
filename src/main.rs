use std::fs;
use std::env;
use std::io;
use std::io::BufRead;

fn main() -> io::Result<()> {
    
    // main loop
    loop {

        let path = env::current_dir()?;

        //get list of folders in current directory
        let mut folders: Vec<fs::DirEntry> = fs::read_dir(path)?.
            filter(|x| x
                .as_ref()
                .unwrap()
                .file_type()
                .unwrap()
                .is_dir())
            .collect();
        
        // show folders to user
        for (pos, e) in folders.iter().enumerate() {
            println!("{}: {:?}", pos, e.file_name());
        }

        // get user option
        let input = io::stdin().lock().lines().next().unwrap().unwrap(); 

        if input == "q" { break; }
        if input.parse::<usize>().is_ok() {
            let option: usize = input.parse().unwrap();
            
            let chosen_folder = folders
                .iter()
                .enumerate()
                .filter(|&(i, _)| i == option)
                .map(|(_, e)| e)
                .collect()
                .iter()
                .first()
                .path();
            
            env::set_current_dir(chosen_folder);
        }
    }
    
    Ok(())
}
