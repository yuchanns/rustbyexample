use std::process::Stdio;
use tokio::io::AsyncBufReadExt;
use tokio::{io::BufReader, process::Command};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new("./assets/slow.bash")
        .stdout(Stdio::piped()) // Can do the same for stderr
        .spawn()
        .expect("cannot spawn");

    let stdout = cmd.stdout.take().expect("no stdout");
    // Can do the same for stderr

    // To print out each line
    let mut reader = BufReader::new(stdout).lines();
    tokio::spawn(async move {
        let status = cmd
            .wait()
            .await
            .expect("child process encountered an error");
        println!("child status was: {}", status);
    });
    while let Some(line) = reader.next_line().await? {
        println!("Line: {}", line);
    }

    Ok(())
}
