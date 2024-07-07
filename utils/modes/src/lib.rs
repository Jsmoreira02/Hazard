use std::net::TcpStream;
use pavao::{SmbClient, SmbCredentials, SmbOptions};
use ftp_client::prelude::*;
use ssh2::Session;
use diesel::prelude::*;
use diesel::{
    pg::PgConnection,
    mysql::MysqlConnection
};
use std::{thread, time};
use std::sync::{Arc, Mutex};

pub fn validate_postgres(target_addr: &str, port: &str, target_username: &str, password: &str, optional_db: &str, delay: u64) -> Result<(), ()> {
    
    let database_url = format!("postgres://{}:{}@{}:{}/{}", target_username, password, target_addr, port, optional_db);

    let _ = time::Instant::now();
    thread::sleep(time::Duration::from_secs(delay));

    match PgConnection::establish(&database_url) {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}

pub fn validate_mysql(target_addr: &str, port: &str, target_username: &str, password: &str, optional_db: &str, delay: u64) -> Result<(), ()> {
    
    let _ = time::Instant::now();
    thread::sleep(time::Duration::from_secs(delay));

    let database_url = format!("mysql://{}:{}@{}:{}/{}", target_username, password, target_addr, port, optional_db);

    match MysqlConnection::establish(&database_url) {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}

pub fn validate_ssh(target_addr: &str, target_username: &str, password: &str, delay: u64) -> Result<(), ()> {

    match TcpStream::connect(target_addr) {
        Ok(tcp) => {
            
            let mut sess = Session::new().unwrap();
            sess.set_tcp_stream(tcp);
            let _ = sess.handshake();

            let _ = time::Instant::now();
            thread::sleep(time::Duration::from_secs(delay));

            let _ = sess.userauth_password(target_username, password);

            if sess.authenticated() {
                Ok(())
            } else {
                Err(())
            }
        }
        Err(e) => {
            println!("Error: {}", e);
            Err(())
        }
    }
}

pub fn validate_smb(target_addr: &str, port: &str, target_username: &str, password: &str, share: &str, workgroup: &str, delay: u64, smb_client_lock: Arc<Mutex<()>>) -> Result<(), ()> {

    let _lock = smb_client_lock.lock().unwrap();

    let client_result = SmbClient::new(
        SmbCredentials::default()
            .server(format!("smb://{}:{}/", target_addr, port))
            .share(share)
            .username(target_username)
            .password(password)
            .workgroup(workgroup),
        SmbOptions::default()
            .case_sensitive(true),
    );

    let _ = time::Instant::now();
    thread::sleep(time::Duration::from_secs(delay));

    match client_result {
        Ok(client) => {
            match client.list_dir("/") {
                Ok(_) => return Ok(()),
                Err(_) => return Err(()),
            }
        }
        Err(e) => {
            println!("Error: {}", e);
            return Err(())
        }
    }
}

pub fn validate_ftp(target_addr: &str, port: u32, target_username: &str, password: &str, delay: u64) -> Result<(), ()> {

    let _ = time::Instant::now();
    thread::sleep(time::Duration::from_secs(delay));

    match Client::connect_with_port(target_addr, port, target_username, password) {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}
