use colored::Colorize;

#[cfg(windows)]
pub fn enable_virtual_terminal_processing() {
    
    use winapi::um::consoleapi::GetConsoleMode;
    use winapi::um::consoleapi::SetConsoleMode;
    use winapi::um::handleapi::INVALID_HANDLE_VALUE;
    use winapi::um::processenv::GetStdHandle;
    use winapi::um::winbase::STD_OUTPUT_HANDLE;
    use winapi::um::wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING;

    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        if handle == INVALID_HANDLE_VALUE {
            return;
        }

        let mut mode: u32 = 0;
        if GetConsoleMode(handle, &mut mode) == 0 {
            return;
        }

        if SetConsoleMode(handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING) == 0 {
            return;
        }
    }
}

pub fn display_helpmsg() -> String {

    let help = format!("
{} ☢ Hazard is a bruteforce tool for the following Service Ports => [ssh, ftp, smb, postgresql, mysql]

    {}:
        [Network Protocol] --> Enter the name of the network protocol to be attacked: ssh, ftp, smb, postgresql, mysql

    {}:
        -t/--target --> Insert Target Address to launch the attack
        -u/--username --> Enter the username value
        -f/--file --> Enter (wordlist) file path for bruteforcing

    {}:
        -p/--port --> Enter the Customized service port
        -n/--number --> Number of authentication threads. Default Value = [8 Threads]
        -w/--waitLogin --> Set Delay (as secs) per login attempt. Default Value = [0s]
        --smb_share --> Enter the SMB folder to check auth attempt
        --showPassword --> Print the current password being tested
        --db --> Enter Optional/Custom Postgresql or MySQL database. Default Value = [postgres/mysql]

    
{} {}: Hazard is a tool designed for fun and for auditing or penetration testing in controlled or authorized environments, 
    and is not intended to be used for malicious or illegal purposes of any kind. 
    Please use it wisely and responsibly. But most of all, have fun :D
    
    ", "[!]".yellow().bold(), "Positional Arguments".cyan().bold(), "Required".cyan().bold(), "Optionals".cyan().bold(), "[!]".yellow().bold(), "Warning".red());

    return help;

}

pub fn banner() {

    #[cfg(windows)]
    control::set_virtual_terminal(true).unwrap();

    let description = "---------------------|> [BruteForce Tool] <|---------------------\n\n";
    let logo = r#"

   ▄█    █▄       ▄████████  ▄███████▄     ▄████████    ▄████████ ████████▄  
  ███    ███     ███    ███ ██▀     ▄██   ███    ███   ███    ███ ███   ▀███ 
  ███    ███     ███    ███       ▄███▀   ███    ███   ███    ███ ███    ███ 
 ▄███▄▄▄▄███▄▄   ███    ███  ▀█▀▄███▀▄▄   ███    ███  ▄███▄▄▄▄██▀ ███    ███ 
▀▀███▀▀▀▀███▀  ▀███████████   ▄███▀   ▀ ▀███████████ ▀▀███▀▀▀▀▀   ███    ███ 
  ███    ███     ███    ███ ▄███▀         ███    ███ ▀███████████ ███    ███ 
  ███    ███     ███    ███ ███▄     ▄█   ███    ███   ███    ███ ███   ▄███ 
  ███    █▀      ███    █▀   ▀████████▀   ███    █▀    ███    ███ ████████▀  
                                                       ███    ███            
    
    "#;

    println!("{} {}", logo.blue().bold(), description.white().bold());
}