use std::{fs, io::Write, os::unix::fs::FileExt};

pub mod common;
pub mod input_process;
pub mod output_process;

struct PrbHeaderEntry {
    id: u32,
    offset: usize
}

fn main() {
    //create output directory
    let output_dir = "output";
    match fs::create_dir(output_dir) {
        Ok(v) => println!("Created output directory"),
        Err(_err) => {}
    };


    let input_dir = "input";
    let input_files = match fs::read_dir(input_dir) {
        Ok(v) => v,
        Err(err) => panic!("Failed to read input directory with error: {}. Ending", err)
    };

    //create output file
    let output_file = match fs::File::create(output_dir.to_owned() + "/output.prb") {
        Ok(v) => v,
        Err(err) => panic!("Failed to acquire output file with error: {}. Ending", err)
    };

    //get header size
    let num_files = match fs::read_dir(input_dir) {
        Ok(v) => v.count(),
        Err(err) => panic!("Failed to read input directory with error: {}. Ending", err)
    };

    //initialize offsets and entries
    let mut data_offset = num_files * std::mem::size_of::<PrbHeaderEntry>();
    let mut header_entries: Vec<PrbHeaderEntry> = Vec::new();
    header_entries.reserve(num_files);

    //iterate files
    for file_result in input_files {
        //verify file
        let file = match file_result {
            Ok(v) => v,
            Err(_err) => continue
        };
        
        println!("Reading file: {}", file.path().display());

        //process data
        let model_data = match input_process::process_glb(&file.path()) {
            Some(v) => v,
            None => {
                println!("Skipping file because it couldn't be loaded");
                continue;
            }
        };

        //create header entry
        header_entries.push( PrbHeaderEntry {
            id: header_entries.len() as u32,
            offset: header_entries.len() * std::mem::size_of::<PrbHeaderEntry>()
        });

        //write data into binary blob
        output_process::write_glb(&output_file, model_data);
    }

    //write number of entries at start of file
    match output_file.write_at(&num_files.to_be_bytes(), 0) {
        Ok(_v) => {},
        Err(err) => panic!("Failed to write header entries with error: {}", err)
    }
}
