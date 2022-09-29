use clap::{ArgGroup, Parser, ValueEnum};

/// Command line arguments
#[derive(Parser, Debug, Clone)]
#[command(name = "wz",author, version, about = "Wortzahl! Count words, fast", long_about = None)]
#[command(group(ArgGroup::new("input").required(false).args(& ["from_stdin", "files"])))]
pub struct Config {
    /// Read file paths from stdin
    ///
    /// When this flag is enabled, wz will treat each line in stdin as a file path
    /// Example:
    ///     $ cat list_files.txt | wz --from-stdin
    #[arg(long)]
    pub from_stdin: bool,

    /// Enables line count
    #[arg(short, long)]
    pub lines: bool,

    /// Enables character count
    #[arg(short, long = "chars")]
    pub characters: bool,

    /// Enables word count
    ///
    /// A word boundary is defined in isspace(3)
    #[arg(short, long)]
    pub words: bool,

    /// Enables byte count
    #[arg(short, long)]
    pub bytes: bool,

    /// Shows the longest line size, in characters
    #[arg(short = 'L', long = "max-line-length")]
    pub max_line_length: bool,
    /// Line break to use
    ///
    /// The kind of line break wz will search for. It can be LF ('\n') or CR
    /// ('\r'). For Windows' CRLF files, either should work fine
    #[arg(short, long, default_value_t, value_enum)]
    pub newline: LineBreak,

    /// File encoding
    ///
    /// The file encoding used on these files
    #[arg(short, long, default_value_t, value_enum)]
    pub encoding: Encoding,

    /// Output format
    ///
    /// Prints the result with the specified format
    #[arg(long, default_value_t, value_enum)]
    pub output: Format,

    /// List of input files to analyze
    ///
    /// If no file is provided, wz will default to stdin input. Conflicts with
    /// `from-stdin`
    #[arg(verbatim_doc_comment)]
    pub files: Vec<String>,
}

/// Available line break options
#[derive(Debug, Clone, Default, ValueEnum)]
pub enum LineBreak {
    #[default]
    LF,
    CR,
}
impl From<LineBreak> for u8 {
    fn from(lb: LineBreak) -> Self {
        match lb {
            LineBreak::LF => b'\n',
            LineBreak::CR => b'\r',
        }
    }
}

/// Supported encodings
#[derive(Debug, Clone, Default, ValueEnum)]
pub enum Encoding {
    #[default]
    UTF8,
    //UTF16,
}

/// Supported output formats
/// https://docs.rs/crate/tabled/
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum Format {
    Ascii,
    Psql,
    Markdown,
    #[default]
    Rounded,
    Extended,

    Json,
}
