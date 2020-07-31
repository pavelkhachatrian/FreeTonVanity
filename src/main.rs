pub mod addr_gen;
pub mod addr_checker;
pub mod file_backend;


extern crate streaming_iterator;

use std::time::Instant;
use std::{thread, iter, time};
use clap::{Arg, App};
use crate::addr_checker::BeautyAddressCheck;
use crate::file_backend::{FileBackend, Connector};
use std::sync::{Mutex, Arc};

#[allow(unused_variables)]
pub fn main() -> Result<(), String> {
    let matches = App::new("Free TON Vanity address generator")
        .version("0.1")
        .author("Pavel @get_username")
        .about("Generating addresses for Free TON with random mnemonic \
                or just random secret, checking, saving(file, redis, postgresql)")
        .arg(Arg::with_name("mnemonic")
            .short("m")
            .long("mnemonic")
            .help("Set true if you need to addresses with random mnemonic seed(much slower if set true)")
            .takes_value(true))
        .arg(Arg::with_name("contract_path")
            .short("c")
            .long("contract")
            .help("Path to tvc file with contract for which addresses will be generated")
            .takes_value(true))
        .arg(Arg::with_name("threads_amount")
            .short("t")
            .long("threads")
            .help("Amount of threads")
            .takes_value(true))
        .arg(Arg::with_name("file_backend")
            .short("f")
            .long("file")
            .help("Path to file where to save results, redis or postgresql connection string")
            .takes_value(true))
        .get_matches();


    let with_mnemonic = matches.value_of("mnemonic").unwrap_or("false")
        .parse::<bool>()
        .map_err(|e| format!("unable to parse \"mnemonic\" arg as bool: {}", e))?;
    let threads_amount = matches.value_of("threads_amount").unwrap_or("1")
        .parse::<usize>()
        .map_err(|e| format!("unable to parse \"threads_amount\" arg as usize: {}", e))?;

    let contract_path = matches.value_of("contract_path").unwrap_or("SetcodeMultisigWallet.tvc");
    let file_backend_path = matches.value_of("file_backend").unwrap_or("addresses.csv");
    let file_backend = FileBackend::from_path(file_backend_path);
    let connector = Arc::new(Mutex::new(file_backend.get_connector()));

    println!("Running Free TON Vanity address generator");
    println!("Use mnemonic seed generator: {}", with_mnemonic);
    println!("Threads amount: {}", threads_amount);
    println!("Contract path: {}", contract_path);
    println!("File connector path: {}", file_backend_path);

    let handles: Vec<_> = (0..threads_amount)
        .map(|_| {
            let conn = connector.clone();
            let contract_path = contract_path.to_string();
            thread::spawn(move || {
                println!("Started!");
                run(conn, contract_path, with_mnemonic);
                println!("Finished!");
            })
        })
        .collect();


    for handle in handles {
        handle.join().unwrap()
    }


    Ok(())
}

fn run(file_backend_connector: Arc<Mutex<Box<dyn Connector>>>, path: String, with_mnemonic: bool) {
    let bas = BeautyAddressCheck::new();
    let mut address_generator = addr_gen::AccountGenerator::from_tvc_file(&path).unwrap();
    let batch_size: u32 = 1000000;
    // for _ in 0..10 {
    loop {
        let batch_time_start = Instant::now();
        for _ in 0..batch_size {
            // let time_start = Instant::now();
            let account;
            if with_mnemonic {
                account = address_generator.generate_account_from_random_seed();
            } else {
                account = address_generator.generate_random_account();
            }
            let id = account.account_id.clone();
            let rule = bas.rule_beauty_address(&id);
            if rule > 0 {
                file_backend_connector.lock().unwrap().push(account, rule);
            }
            // let elapsed_time = time_start.elapsed();
            // println!("{}", elapsed_time.as_micros());

        }
        let batch_elapsed_time = batch_time_start.elapsed();
        println!("TIME FOR {} addresses in one thread: {}", batch_size, batch_elapsed_time.as_secs());
    }
    file_backend_connector.lock().unwrap().save();
}

