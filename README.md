![Captura_de_imagem_20240707_125436-removebg-preview](https://github.com/Jsmoreira02/Hazard/assets/103542430/c2f4971c-aa5b-4eff-9cd5-c9be68aeb223)


<div>
    <img src="https://img.shields.io/badge/Language%20-Rust-orange.svg" style="max-width: 100%;">
    <img src="https://img.shields.io/badge/Target OS%20-Linux, Windows-blue.svg" style="max-width: 100%;">
    <img src="https://img.shields.io/badge/Cargo builds%20-ftp_client, ssh2, pavao, diesel-beige.svg" style="max-width: 100%;">
    <img src="https://img.shields.io/badge/Type%20-Network, Bruteforce Exploit-black.svg" style="max-width: 100%;">
    <img src="https://img.shields.io/badge/Command Line tools%20-teste?style=flat-square style="max-width: 100%;">
</div>

# Hazard 

Hazard is dictionary bruteforce tool, built for a simple and user-friendly interface, supporting the most common (and sensitive) network protocols such as `ssh, ftp, samba, mysql, postgresql`. 

> "A brute force attack is a hacking method that uses trial and error to crack passwords, login credentials, and encryption keys. It is a simple yet reliable tactic for gaining unauthorized access to individual accounts and organizations’ systems and networks. The hacker tries multiple usernames and passwords, often using a computer to test a wide range of combinations, until they find the correct login information."


## Usage:
***Example:*** `hazard ssh -t targetIP -f ../../rockyou.txt --showPasswords -n 10`

![hazard-ezgif com-video-to-gif-converter](https://github.com/Jsmoreira02/Hazard/assets/103542430/184da502-43ad-41ad-a3a7-01f8a0076e57)


## Installation:

```bash
  git clone https://github.com/Jsmoreira02/Hazard.git
  cd Hazard
  bash install_dependecies.sh
```

or

```bash
  curl -o install_dependecies.sh https://raw.githubusercontent.com/Jsmoreira02/Hazard/main/install_dependecies.sh && bash install_dependecies.sh
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

