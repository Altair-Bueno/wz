use clap::{ArgGroup, Parser, ValueEnum};

#[derive(Parser, Debug, Clone)]
#[clap(name = "wz",author, version, about = "Wortzahl! Count words, fast", long_about = None)]
#[clap(group(ArgGroup::new("input").required(false).args(& ["from-stdin", "files"])))]
pub struct Config {
    /// Read file paths from stdin
    ///
    /// When this flag is enabled, wz will treat each line in stdin as a file path
    /// Example:
    ///     $ cat list_files.txt | wz --from-stdin
    #[clap(long, verbatim_doc_comment)]
    pub from_stdin: bool,

    /// Enables line count
    #[clap(short, long, verbatim_doc_comment)]
    pub lines: bool,

    /// Enables character count
    #[clap(short, long = "chars", verbatim_doc_comment)]
    pub characters: bool,

    /// Enables word count
    ///
    /// A word boundary is defined in isspace(3)
    #[clap(short, long, verbatim_doc_comment)]
    pub words: bool,

    /// Enables byte count
    #[clap(short, long, verbatim_doc_comment)]
    pub bytes: bool,

    //     /// Shows the longest line size
    //     #[clap(short = 'L', long, verbatim_doc_comment)]
    //     pub line_length: bool,
    /// Line break to use
    ///
    /// The kind of line break wz will search for. It can be LF ('\n') or CR
    /// ('\r'). For Windows' CRLF files, either should work fine
    #[clap(short, long, default_value_t, value_enum)]
    pub newline: LineBreak,

    /// File encoding
    ///
    /// The file encoding used on these files
    #[clap(short, long, default_value_t, value_enum)]
    pub encoding: Encoding,

    /// JSON output
    ///
    /// Print results on JavaScript Object Notation
    #[clap(long)]
    pub json: bool,

    /// List of input files to analyze
    ///
    /// If no file is provided, wz will default to stdin input. Conflicts with
    /// `from-stdin`
    #[clap()]
    pub files: Vec<String>,
}
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
#[derive(Debug, Clone, Default, ValueEnum)]
pub enum Encoding {
    #[default]
    UTF8,
    //UTF16,
}

pub fn load() -> Config {
    let mut config = Config::parse();
    if ![config.bytes, config.characters, config.words, config.lines]
        .into_iter()
        .any(|x| x)
    {
        config.lines = true;
        config.words = true;
        config.bytes = true;
    }
    config
}
