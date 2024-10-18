use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[arg(
        short,
        long,
        help = "Print NUM lines of trailing context after matching lines.
              Places a line containing a group separator (--) between
              contiguous groups of matches."
    )]
    pub after: Option<usize>,

    #[arg(
        short,
        long,
        help = "Print NUM lines of leading context before matching lines.
              Places a line containing a group separator (--) between
              contiguous groups of matches."
    )]
    pub before: Option<usize>,

    #[arg(
        short('C'),
        long,
        help = "Print NUM lines of output context.  Places a line
              containing a group separator (--) between contiguous
              groups of matches."
    )]
    pub context: Option<usize>,

    #[arg(
        short,
        long,
        help = "Suppress normal output; instead print a count of matching
              lines for each input file."
    )]
    pub count: bool,

    #[arg(
        short('i'),
        long("ignore-case"),
        help = "Ignore case distinctions in patterns and input data, so
              that characters that differ only in case match each other."
    )]
    pub ignore: bool,

    #[arg(
        short('v'),
        long,
        help = "Invert the sense of matching, to select non-matching
              lines."
    )]
    pub invert: bool,

    #[arg(
        short('F'),
        long,
        help = "Interpret PATTERN as fixed strings, not regular
              expressions."
    )]
    pub fixed: bool,

    #[arg(
        short('n'),
        long,
        help = "Prefix each line of output with the 1-based line number
              within its input file."
    )]
    pub line_num: bool,

    #[arg(
        required = true,
        help = "PATTERN is one or more
       patterns separated by newline characters, and grep prints each
       line that matches a pattern."
    )]
    pub pattern: String,

    #[arg()]
    pub file: Option<String>,
}
