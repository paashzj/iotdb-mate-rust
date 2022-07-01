use std::env;
use std::process::Stdio;

use tokio::io::AsyncReadExt;
use tokio::process::Command;

#[tokio::main]
async fn main() {
    let result = env::var("IOTDB_HOME").unwrap();
    let script_path = result + "/mate/scripts/start-iotdb-standalone.sh";
    let fail_msg = "failed to execute process";
    let mut cmd = Command::new("/bin/bash")
        .arg(script_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect(fail_msg);

    let stdout = &mut String::new();
    cmd.stdout.take().unwrap().read_to_string(stdout).await.unwrap();
    println!("stdout : {:?}", stdout);

    let stderr = &mut String::new();
    cmd.stderr.take().unwrap().read_to_string(stderr).await.unwrap();
    println!("stderr : {:?}", stderr);
}
