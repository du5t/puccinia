use std::process::{Command, Child};
use std::error::Error;
use std::collections::HashMap;

trait TorControl<'a> {
    // TODO make result types explicit re: E0277
    fn start(&'a mut self, cfg_dict: Option<HashMap<&str, String>>)
             -> Result<Command, Error>;
    fn stop(&'a mut self) -> Result<(), Error>;
}

struct TorProcess {
    cmd_path: String,
    cmd_args: Vec<String>,
    torrc_path: String,
    min_bootstrap: u8,
    timeout_secs: u8,
    kill_on_disconnect: bool, // whether or not the process should be tied to
                              // this controller
    process: Option<Child> // resolves if started
}

impl TorProcess {
    fn new(proc_opts: Option<HashMap<&str, String>>) -> TorProcess {
        TorProcess {
            cmd_path: proc_opts.get("ch").or("tor"),
            cmd_args: proc_opts.get("cmd_args").or(""),
            torrc_path: proc_opts.get("torrc_path").or(""),
            min_bootstrap: proc_opts.get("min_bootstrap").or(100),
            timeout_secs: proc_opts.get("timeout_secs").or(90),
            kill_on_disconnect: proc_opts.get("kill_on_disconnect").or(false)
        };
    }
}

// TODO validation on args
impl<'a> TorControl<'a> for TorProcess {
    fn start(&'a mut self, cfg_dict: Option<HashMap<&str, String>>)
             -> Result<Child, Error> {
        if self.process.is_none() {
            let torrc_opt = "";
            if self.torrc_path.len > 0 {
                torrc_opt = "-f " + self.torrc_path;
            }
            Command::new(self.cmd_path)
                .arg(torrc_opt).arg(self.cmd_args).spawn();
        } else {
            Err("I already have a Tor process running.")
        }
    }
    fn stop(&'a mut self) -> Result<(), Error> {
        // checks if there is a process running first
        self.process.ok_or(Err("No process running.")).kill();
    }
}
