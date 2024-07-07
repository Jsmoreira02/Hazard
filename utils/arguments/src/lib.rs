use clap::{Arg, Command};

#[derive(Debug)]
pub struct Arguments {
    pub protocol: String,
    pub target_addr: String,
    pub target_username: String,
    pub file_path: String,
    pub custom_port: String,
    pub smb_share: Option<String>,
    pub db: Option<String>,
    pub tasks: String,
    pub delay: String,
    pub show_password_testing: String,
}

pub fn default_port(protocol: &str) -> &str {
    match protocol {
        "ssh" => "22",
        "ftp" => "21",
        "smb" => "445",
        "pg" => "5432",
        "mysql" => "3306",
        _ => "",
    }
}

pub fn parse_arguments() -> Arguments {
    let arguments = Command::new("Hazard")
        .override_help(display::display_helpmsg())
        .arg(
            Arg::new("Network Protocol")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::new("Target address")
                .short('t')
                .long("target")
                .required(true),
        )
        .arg(
            Arg::new("Target Username")
                .short('u')
                .long("username")
                .required(true),
        )
        .arg(
            Arg::new("Wordlist File")
                .short('f')
                .long("file")
                .required(true),
        )
        .arg(
            Arg::new("Custom Port")
                .short('p')
                .long("port"),
        )
        .arg(
            Arg::new("SMB share")
                .long("smb_share")
                .default_value("")
                .hide_default_value(true)
                .required_if_eq("Network Protocol", "smb"),
        )
        .arg(
            Arg::new("SQL database")
                .long("db")
                .default_value("")
                .hide_default_value(true),
        )
        .arg(
            Arg::new("Request Numbers")
                .short('n')
                .long("number")
                .default_value("8")
                .hide_default_value(true),
        )
        .arg(
            Arg::new("Delay Time")
                .short('w')
                .long("waitLogin")
                .default_value("0")
                .hide_default_value(true),
        )
        .arg(
            Arg::new("Show Password Testing")
                .long("showPassword")
                .default_missing_value("true")
                .default_value("")
                .num_args(0..=1)
                .require_equals(true)
                .hide_default_value(true),
        )
        .get_matches();

    let protocol = arguments.get_one::<String>("Network Protocol").unwrap().to_string();
    let file_path = arguments.get_one::<String>("Wordlist File").unwrap().to_string();
    let target_username = arguments.get_one::<String>("Target Username").unwrap().to_string();
    let target_addr = arguments.get_one::<String>("Target address").unwrap().to_string();
    let smb_share = arguments.get_one::<String>("SMB share").map(|s| s.to_string());
    let tasks = arguments.get_one::<String>("Request Numbers").unwrap().to_string();
    let delay = arguments.get_one::<String>("Delay Time").unwrap().to_string();
    let show_password_testing = arguments.get_one::<String>("Show Password Testing").unwrap().to_string();
    let custom_port = arguments.get_one::<String>("Custom Port").map(|p| p.to_string());
    let db = arguments.get_one::<String>("SQL database").map(|db| db.to_string());

    Arguments {
        protocol: protocol.clone(),
        target_addr,
        target_username,
        file_path,
        custom_port: custom_port.unwrap_or_else(|| default_port(&protocol).to_string()),
        smb_share,
        db,
        tasks,
        delay,
        show_password_testing,
    }
}
