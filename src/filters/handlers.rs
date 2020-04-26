use std::convert::Infallible;
use std::fs;
use std::process::Command;
use warp::http::StatusCode;

use crate::errors::*;

pub async fn update_ip(addr: String) -> std::result::Result<impl warp::Reply, Infallible> {
    match call_nsupdate(addr) {
        Err(e) => {
            use error_chain::ChainedError;

            log::error!("updating dns records: {}", e.display_chain());
            return Ok(StatusCode::INTERNAL_SERVER_ERROR);
        }
        Ok(_) => Ok(StatusCode::ACCEPTED),
    }
}

fn call_nsupdate(addr: String) -> Result<()> {
    let update_cmds = format!(
        "
update delete *.choo.dev A
update add *.choo.dev 86400 A {}
send
",
        addr
    );

    fs::write("/tmp/nsupdate_cmd", update_cmds).chain_err(|| "writing nsupdate cmd")?;

    let cmd_output = Command::new("nsupdate")
        .args(vec!["-l", "-4", "/tmp/nsupdate_cmd"])
        .output()
        .chain_err(|| "running nsupdate cmd")?;

    if !cmd_output.status.success() {
        bail!(
            "nsupdate command failed:\nstdout: {}\nstderr: {}",
            String::from_utf8(cmd_output.stdout).unwrap(),
            String::from_utf8(cmd_output.stderr).unwrap()
        );
    }

    Ok(())
}
