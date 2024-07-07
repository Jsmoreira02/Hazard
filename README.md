<div>
    <img src="https://img.shields.io/badge/Language%20-Rust-orange.svg" style="max-width: 100%;">
    <img src="https://img.shields.io/badge/Target OS%20-Linux, Windows-blue.svg" style="max-width: 100%;">
    <img src="https://img.shields.io/badge/Cargo builds%ftp_client, ssh2, pavao, diesel-beige.svg" style="max-width: 100%;">
    <img src="https://img.shields.io/badge/Type%20-Network Protocol, Bruteforce Exploit-black.svg" style="max-width: 100%;">
    <img src="https://img.shields.io/badge/Command Line tools%20-teste?style=flat-square style="max-width: 100%;">
</div>

# Hazard 

Hazard is dictionary bruteforce tool, built for a simple and user-friendly interface, supporting the most common (and sensitive) network protocols such as `ssh, ftp, samba, mysql, postgresql`. 

> "A brute force attack is a hacking method that uses trial and error to crack passwords, login credentials, and encryption keys. It is a simple yet reliable tactic for gaining unauthorized access to individual accounts and organizations’ systems and networks. The hacker tries multiple usernames and passwords, often using a computer to test a wide range of combinations, until they find the correct login information."


##Usage:
***Example:*** `hazard ssh -t targetIP -f ../../rockyou.txt --showPasswords -n 10`


## Installation:

```bash
  git clone https://github.com//Jsmoreira02/Hazard.git
  cd Hazard
  cargo build --release --manifest-path Cargo.toml --target-dir .
```

or

```bash
  cargo install --git https://github.com//Jsmoreira02/hazard.git Hazard
```

## More info:

- **Colored Documentation** => [Colored](https://crates.io/crates/colored)
- **Clap Documentation** => [Clap](https://docs.rs/clap/latest/clap/)
- **Rust SSH Documentation** => [ssh2](https://docs.rs/ssh2/latest/ssh2/)
- **Rust FTP Documentation** => [ftp-client](https://docs.rs/ftp/latest/ftp/)
- **Pavao (Rust Samba client)** => [Pavao](https://docs.rs/pavao/latest/pavao/)
- **Rust SQL database client** => [diesel](https://docs.rs/diesel/latest/diesel/)


# Warning:    
> I am not responsible for any illegal use or damage caused by this tool. It was written for fun, not evil and is intended to raise awareness about cybersecurity.


***Have a good hack :D***

