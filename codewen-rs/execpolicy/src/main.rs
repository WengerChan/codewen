use anyhow::Result;
use clap::Parser;
use codewen_execpolicy::ExecPolicyCheckCommand;

/// CLI for evaluating exec policies
#[derive(Parser)]
#[command(name = "codewen-execpolicy")]
enum Cli {
    /// Evaluate a command against a policy.
    Check(ExecPolicyCheckCommand),
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli {
        Cli::Check(cmd) => cmd.run(),
    }
}
