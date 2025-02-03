use std::{fs, os::unix::fs::FileExt};

pub mod common;
pub mod input_process;
pub mod output_process;

struct PrbHeaderEntry {
    _id: u64,
    _offset: u64
}

impl PrbHeaderEntry {
    pub fn as_slice(&self) -> &[u8; std::mem::size_of::<PrbHeaderEntry>()] {
        unsafe {
            &*(self as *const PrbHeaderEntry as *const [u8; std::mem::size_of::<PrbHeaderEntry>()])
        }
    }
}

fn main() {
    //create output directory
    let output_dir = "output";
    match fs::create_dir(output_dir) {
        Ok(_v) => println!("Created output directory"),
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
    let mut header_offset = std::mem::size_of::<usize>() as u64; //first 8 bytes takes up number of entries
    let mut data_offset = (num_files * std::mem::size_of::<PrbHeaderEntry>()) as u64 + header_offset;
    let mut data_entries_count: u64 = 0;

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
        let header_entry = PrbHeaderEntry {
            _id: data_entries_count,
            _offset: data_offset as u64
        };
        data_entries_count += 1;

        //write entry to file
        match output_file.write_at(header_entry.as_slice(), header_offset) {
            Ok(_v) => {},
            Err(err) => panic!("Failed to write header entry to file with error: {}", err)
        }
        header_offset += std::mem::size_of::<PrbHeaderEntry>() as u64;


        //write data into binary blob
        data_offset += output_process::write_glb(&output_file, model_data, data_offset);
    }

    //write number of entries at start of file
    match output_file.write_at(&data_entries_count.to_le_bytes(), 0) {
        Ok(_v) => {},
        Err(err) => panic!("Failed to write header entries with error: {}", err)
    }
}
