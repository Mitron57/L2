mod shell;

use crate::shell::Shell;

fn main() {
    let mut shell = Shell::new();
    shell.listen_to_stdin();
}
