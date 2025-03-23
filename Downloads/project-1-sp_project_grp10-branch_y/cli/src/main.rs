
use clap::{Args, Parser, Subcommand};
//use rand::Rng;
use std::num::NonZeroU8;
//use num::Integer;
//use tracing_subscriber::fmt;

//gen-passwords commands
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct ProjectArgs {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    GenPasswords(PasswordArgs), 
    
    GenHashes(GenhashesArgs),

    DumpHashes(DumphashesArgs)
}

#[derive(Debug, Args)]
struct PasswordArgs {
    /// Number of chars to be used in generated passowrds
    #[clap(long, default_value= "4", value_parser = clap::value_parser!(u8).range(1..))]
    chars: u8,

    /// Output file where contents of the command needs to be written
    #[clap(long)]
    out_file: Option<String>,

    /// The number of threads to process numbers on.
    #[clap(long, default_value= "1", value_parser = threads_in_range)]
    threads: NonZeroU8,

    /// Number of passwords to generate
    #[clap(long, value_parser = num_in_range)]
    num: usize,
}

fn num_in_range(gen_num: &str) -> Result<usize, String> {
    let gen_num: usize = gen_num
        .parse()
        .map_err(|_| format!("`{gen_num}` is invalid"))?;
    if gen_num > 0 {
        Ok(gen_num)
    } else {
        Err(format!("Number should be greater than zero"))
    }
}

fn threads_in_range(threads: &str) -> Result<NonZeroU8, String> {
    let threads: NonZeroU8 = threads
        .parse()
        .map_err(|_| format!("`{threads}` is invalid"))?;
    Ok(threads)
}


#[derive(Debug, Args)]
struct GenhashesArgs {
    /// Input file where contents needs to be read from
    #[clap(long)]
    in_file: String,

    /// Output file where contents of the command needs to be written
    #[clap(long)]
    out_file: String,

    /// The number of threads to process numbers on.
    #[clap(long, default_value= "1", value_parser = threads_in_range)]
    threads: NonZeroU8,

    /// State an algorithm to be used for hashing
    #[clap(long)]
    algorithm: String,
}

#[derive(Debug, Args)]
struct DumphashesArgs {
    /// Input file where contents needs to be read from
    #[clap(long)]
    in_file: String,
}



fn main() -> color_eyre::Result<()> {
    let args = ProjectArgs::parse();

    //println!("{args:?}")

    match args.command {
        Commands::GenPasswords(args) => hashassin_core::gen_passwords::generate_passwords(
            args.chars,
            &args.out_file,
            args.threads,
            args.num,
        )?,
        Commands::GenHashes(args) => hashassin_core::gen_hashes::generate_hashes(
            &args.in_file,
            &args.out_file,
            args.threads,
            &args.algorithm
        )?,
        Commands::DumpHashes(args) => hashassin_core::dump_hashes::dump_hashes(&args.in_file)?
    }

    Ok(())
}


