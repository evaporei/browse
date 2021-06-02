use std::env;
use std::process::{self, Command, ExitStatus};

// TODO: refactor all of this to work as lib as well

const HELP: &str = "\
browse - CLI tool to open an URL in your browser
USAGE:
    browse <URL>
FLAGS:
    -h, --help       Prints help information
PARAMETERS:
    <URL>            An URL like https://crates.io

GitHub repo: https://github.com/otaviopace/browse
";

/// For information on opening for each platform:
/// - windows: https://stackoverflow.com/a/49115945
/// - others: https://dwheeler.com/essays/open-files-urls.html

#[cfg(target_os = "windows")]
const BROWSER_COMMAND: &str = "rundll32.exe";
#[cfg(target_os = "macos")]
const BROWSER_COMMAND: &str = "open";
#[cfg(target_os = "linux")]
const BROWSER_COMMAND: &str = "xdg-open";

#[cfg(target_os = "windows")]
const OPEN_ARGS: &[&str] = &["url.dll,FileProtocolHandler"];
#[cfg(target_os = "macos")]
const OPEN_ARGS: &[&str] = &[];
#[cfg(target_os = "linux")]
const OPEN_ARGS: &[&str] = &[];

const DEFAULT_FAILURE_MESSAGE: &str = "Failed to open browser";

fn exit(message: Option<&str>) -> ! {
    match message {
        Some(message) => eprintln!("{}, error message: {}", DEFAULT_FAILURE_MESSAGE, message),
        None => eprintln!("{}", DEFAULT_FAILURE_MESSAGE),
    };

    process::exit(1);
}

enum Cmd {
    Help,
    Open(String),
}

impl Cmd {
    fn execute(self) -> Result<(), CmdError> {
        match self {
            Cmd::Help => Self::help(),
            Cmd::Open(url) => Self::open(&url),
        }
    }

    fn help() -> Result<(), CmdError> {
        println!("{}", HELP);
        Ok(())
    }

    fn open(url: &str) -> Result<(), CmdError> {
        let status_result = Command::new(BROWSER_COMMAND)
            .args(OPEN_ARGS)
            .arg(url)
            .status();

        if let Err(ref error) = status_result {
            return Err(CmdError::Command(error.to_string()));
        }

        if matches!(status_result, Ok(status) if !status.success()) {
            return Err(CmdError::ExitCode(status_result.unwrap()));
        }

        Ok(())
    }
}

// TODO: better naming
enum CmdError {
    Parse(String),
    Command(String),
    ExitCode(ExitStatus),
}

use std::fmt;
impl fmt::Display for CmdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Parse(s) => write!(f, "{}", s),
            Self::Command(s) => write!(f, "{}", s),
            Self::ExitCode(exit_status) => write!(f, "{}", exit_status),
        }
    }
}

fn parse_args(args: &[String]) -> Result<Cmd, CmdError> {
    if args.len() != 1 {
        return Err(CmdError::Parse(
            "There should be only one argument to `browse`, either `--help`/`-h` or an `URL`"
                .into(),
        ));
    }

    if args[0] == "--help" || args[0] == "-h" {
        return Ok(Cmd::Help);
    }

    Ok(Cmd::Open(args[0].to_owned()))
}

fn run() -> Result<(), CmdError> {
    let args: Vec<String> = env::args().skip(1).collect();

    let cmd = parse_args(&args)?;

    cmd.execute()
}

fn main() {
    run().unwrap_or_else(|e| exit(Some(&e.to_string())))
}
