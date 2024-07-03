use clap::Parser;
use mpi::environment::Universe;
use mpi::topology::SimpleCommunicator;
use mpi::traits::*;

mod utils;
use mpi::Rank;
use utils::count_lines;
use utils::hash_string;
use utils::read_lines;
use utils::split_chunks;

//Init MPI
// const  UNIVERSE = mpi::initialize().unwrap();
// const WORLD = UNIVERSE.world();
// const SIZE = WORLD.size();
// const RANK = WORLD.rank();

/// Brutus Password Cracker
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Password hash to crack
    #[arg(short, long)]
    password: String,
    /// Password File
    #[arg(short, long)]
    file: String,
}

struct MPI {
    // universe: Option<mpi::environment::Universe>,
    world: SimpleCommunicator,
    size: mpi::topology::Rank,
    rank: mpi::topology::Rank,
}

impl MPI {
    pub fn initialize() -> Self {
        // Initialize the MPI environment
        let universe = mpi::initialize().unwrap();

        // Get the world communicator
        let world = universe.world();

        // Get the size of the world communicator
        let size = world.size();

        // Get the rank of the current process
        let rank = world.rank();

        // Return the MPI struct instance
        MPI {
            // universe: Some(universe),
            world,
            size,
            rank,
        }
    }
}

fn brute_force(mpi_instance: &MPI, filename: &String, password: &String, chunk_size: &u16) {
    
    match mpi_instance.rank {
        0 => {
            println!("Master Process");

            // Sending the Password Hash to Child Processes
            let msg = String::from(password);
            let msg_btyes = msg.as_bytes();
            for rank in 0..mpi_instance.size {
                mpi_instance.world.process_at_rank(rank).send(msg_btyes);
            }

            //Spliting the file into Chunks
            let line_count = count_lines(filename);
            let chunks_queue = split_chunks(&line_count, chunk_size);
            println!("Chunks: {:?}", chunks_queue);
        }
        _ => {
            println!("Hello from Child Process {}!", mpi_instance.rank);
            let (msg, _) = mpi_instance.world.any_process().receive_vec::<u8>();
            let password = String::from_utf8(msg).expect("Error Converting to Text");
            println!("Process {} got message: {}.", mpi_instance.rank, password);
        }
    }
    // let count: u128 = count_lines(filename);
    // println!("Total Lines: {count}");

    // let lines: Vec<String> = read_lines(filename, 10, 20);
    // println!("{lines:?}");

    // println!("RANK: {:?}", rank);
}

fn main() {
    let args = Args::parse();
    let file_path = args.file;
    let password_hash = args.password;
    let chunk_size: u16 = 333;
    // Initialize the MPI struct using the new initialize method
    let mpi_instance = MPI::initialize();

    // Example usage: printing the rank and size
    println!("Rank: {}, Size: {}", mpi_instance.rank, mpi_instance.size);
    brute_force(&mpi_instance, &file_path, &password_hash, &chunk_size);

    // let line_count: u128 = 1000;

    // let chunks = split_chunks(&line_count, &chunk_size);
    // println!("Chunks: {:?}", chunks);

    // //Init MPI
    // let universe = mpi::initialize().unwrap();
    // let world = universe.world();
    // let size = world.size();
    // let rank = world.rank();

    // let args = Args::parse();
    // println!("Hash: {} File: {}", args.password, args.file);
    // let file_path = args.file;
    // let password_hash = args.password;
    // brute_force(rank, &file_path, &password_hash);

    // match rank {
    //     0 => {
    //         let msg = String::from(password_hash);
    //         let msg_btyes = msg.as_bytes();
    //         for rank in 0..size {
    //             world.process_at_rank(rank).send(msg_btyes);
    //         }
    //         // let msg = vec![4.0f64, 8.0, 15.0];
    //         // println!("Hello from Parent Process {}!", rank);
    //         // world.process_at_rank(rank + 1).send(&msg[..]);
    //     }
    //     _ => {
    //         println!("Hello from Child Process {}!", rank);
    //         let (msg, status) = world.any_process().receive_vec::<u8>();
    //         let password = String::from_utf8(msg).expect("Error Converting to Text");
    //         println!("Process {} got message: {}.", rank, password);
    //     } // 1 => {
    //       //     let (msg, status) = world.any_process().receive_vec::<f64>();
    //       //     println!(
    //       //         "Process {} got message {:?}.\nStatus is: {:?}",
    //       //         rank, msg, status
    //       //     );
    //       // }
    //       // _ => unreachable!(),
    // }
}
