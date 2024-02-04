// use std::env;
use std::process::Command;

// pub enum OsType {
//     MacOS,
//     Windows,
//     Linux,
// }

pub fn check_if_docker_is_installed() -> Result<(), String> {
    let _docker = Command::new("docker")
        .arg("--version")
        .output()
        .expect("failed to execute process");

    if _docker.status.success() {
        return Ok(());
    } else {
        return Err("Docker is not installed".to_owned());
    }
}

pub fn check_if_docker_is_running() -> Result<(), String> {
    let _docker = Command::new("docker")
        .arg("info")
        .output()
        .expect("failed to execute process");

    // read the output
    let _docker_info = String::from_utf8_lossy(&_docker.stdout);

    if _docker.status.success() {
        // let mut msg = "Docker is running".to_owned();
        // msg.push_str(&_docker_info);
        // return msg;

        return Ok(());
    } else {
        return Err("Docker is not running".to_owned());
    }
}

// fn _getos() -> OsType {
//     let os = env::consts::OS;
//     println!("Current OS is {}", os);
//     if os == "macos" {
//         return OsType::MacOS;
//     } else if os == "windows" {
//         return OsType::Windows;
//     } else {
//         return OsType::Linux;
//     }
// }

// fn _check_linux_package_manager() -> String {
//     let _apt = Command::new("apt-get")
//         .arg("--version")
//         .output()
//         .expect("failed to execute process");

//     let _yum = Command::new("yum")
//         .arg("--version")
//         .output()
//         .expect("failed to execute process");

//     if _apt.status.success() {
//         return "apt-get".to_owned();
//     } else if _yum.status.success() {
//         return "yum".to_owned();
//     } else {
//         return "unknown".to_owned();
//     }
// }
// fn _check_package_manager() -> String {
//     return match _getos() {
//         OsType::Linux => (_check_linux_package_manager.to_owned())(),
//         OsType::MacOS => "brew".to_owned(),
//         OsType::Windows => "choco".to_owned(),
//     };
// }

// fn _get_install_script() -> String {
//     return match _getos() {
//         OsType::Linux => {
//             "curl -fsSL https://get.docker.com -o get-docker.sh && sh get-docker.sh".to_owned()
//         }
//         OsType::MacOS => {
//             "curl -fsSL https://get.docker.com -o get-docker.sh && sh get-docker.sh".to_owned()
//         }
//         OsType::Windows => {
//             "Invoke-WebRequest -Uri https://get.docker.com -OutFile get-docker.ps1 -UseBasicParsing"
//                 .to_owned()
//         }
//     };
// }
// TODO: Implement the install_docker engine
// pub fn install_docker() {
//     let _install_script = _get_install_script();

//     // check if curl is installed
//     let _curl = Command::new("curl")
//         .arg("--version")
//         .output()
//         .expect("failed to execute process");

//     if !_curl.status.success() {
//         println!("curl or wget is not installed, please install curl or wget and try again.");
//         return;
//     }

//     let _install = Command::new("sh")
//         .arg("-c")
//         .arg(_install_script)
//         .output()
//         .expect("failed to execute process");
// }
