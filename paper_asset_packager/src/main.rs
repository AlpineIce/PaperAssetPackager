use std::fs;

pub mod input_process;
pub mod log;


fn main() {
    //verify directory exists
    let cwd = std::env::current_dir().expect("Couldn't retrieve current working directory");
    
    println!("{}", cwd.display());
    let input_dir = "input";
    let input_files = match fs::read_dir(input_dir) {
        Ok(v) => v,
        Err(_err) => panic!("Failed to read input directory. Ending")
    };

    //iterate files
    for file_result in input_files {
        //verify file
        let file = match file_result {
            Ok(v) => v,
            Err(_err) => continue
        };
        
        log::log(log::LogType::INFO, &file.path().display());
        println!("{}", file.path().display());

        //process data
        let model_data = input_process::process_glb(&file.path());

        //write data into binary blob
    }
}
