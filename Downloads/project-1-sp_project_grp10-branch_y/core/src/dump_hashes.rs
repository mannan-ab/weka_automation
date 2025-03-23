use crate::DumpHashError;

use hex;
use std::{
    fs,
    fs::File,
    io::{self, BufReader, BufWriter, Read, Write},
}; 
//use tracing::{debug, info, instrument, warn};

pub fn dump_hashes(in_file: &str) -> Result<(), DumpHashError> {
    let file = File::open(in_file)?;
    let mut reader = BufReader::new(file);

    // Line1 ->Byte 1: VERSION
    let mut version = [0u8; 1];
    reader.read_exact(&mut version)?;
    //println!("{:?}",version);
    let version = version[0];

    // Line2 ->Byte 2: ALGORITHM LENGTH
    let mut algorithm_len = [0u8; 1];
    reader.read_exact(&mut algorithm_len)?;
    let algorithm_len = algorithm_len[0] as usize;

    // Line2 ->Bytes 3 to (3 + algo_len - 1): ALGORITHM
    let mut algorithm = vec![0u8; algorithm_len];
    reader.read_exact(&mut algorithm)?;
    let algorithm = String::from_utf8(algorithm)?;


    // Line3 ->Byte (4): PASSWORD LENGTH
    let mut password_length = [0u8; 1];
    reader.read_exact(&mut password_length)?;
    let password_length = password_length[0];

    // Line 4 ->Read the remaining bytes
    let mut hash_data = Vec::new();
    reader.read_to_end(&mut hash_data)?;

    // Get hash size length in bytes based on algorithm
    let hash_len = match algorithm.as_str() {
        "md5" => 16,
        "sha256" => 32,
        "sha3_512" => 64,
        "scrypt" => 88, //88bytes

        _ => {
            return Err(DumpHashError::Basic(format!(
                "Unknown algorithm: {}",
                algorithm
            )));
        }
    };

    // Extra- Credit, Ensure the data length has correct bytes
    // if hash_data.len() % hash_len != 0 {
    //     return Err(DumpHashError::Basic(format!(
    //         "Hash data length doesn't contain correct number of bytes",
    //     )));
    // }

    // Output to stdout

    // writeln!(io::stdout(), "VERSION: {}", version)?;
    // writeln!(io::stdout(), "ALGORITHM: {}", algorithm)?;
    // writeln!(io::stdout(), "PASSWORD LENGTH: {}", password_length)?;

    //For testing
    let file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("dump-output.txt")?;

    let mut file = BufWriter::new(file);
    writeln!(file, "VERSION {}", version)?;
    writeln!(file, "ALGORITHM: {}", algorithm)?;
    writeln!(file, "PASSWORD LENGTH: {}", password_length)?;

    for chunk in hash_data.chunks_exact(hash_len) {
        if algorithm == "scrypt" {
            //println!("chunck:{:?}",chunk);

            let string_output = String::from_utf8(chunk.to_vec());

            match string_output {
                Ok(string) => {
                    //writeln!(io::stdout(), "{}", string)?;
                    writeln!(file, "{}", string)?;
                }
                Err(e) => {
                    return Err(DumpHashError::Basic(format!(
                        "Coversion Error from utf8 {e:?}"
                    )));
                }
            }
        } else {
            let hex_output = hex::encode(chunk);
            //writeln!(io::stdout(), "{}", hex_output)?;
            writeln!(file, "{}", hex_output)?;
        }
    }

    io::stdout().flush()?;

    Ok(())
}
