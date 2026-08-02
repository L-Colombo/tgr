use clap::{Args, Parser, Subcommand, ValueHint, builder::styling};

const STYLES: styling::Styles = styling::Styles::styled()
    .header(styling::AnsiColor::Green.on_default().bold())
    .usage(styling::AnsiColor::Green.on_default().bold())
    .literal(styling::AnsiColor::Blue.on_default().bold())
    .placeholder(styling::AnsiColor::Cyan.on_default());

// Commands and their arguments
// Commands

#[derive(Parser)]
#[command(name = "tgr")]
#[command(
    version,
    about = "Manage `.org` files' tags from the CLI",
    long_about = "Search and visualize tags in your `.org` files"
)]
#[command(styles = STYLES)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Print the number of tags that matching a pattern
    #[clap(alias = "c")]
    Count(CountArgs),
    /// Locate the files that contain a tag matching a pattern
    #[clap(aliases = &["l", "loc"])]
    Locate(LocateArgs),
    /// Refile org trees that have tags that match a pattern
    #[clap(aliases = &["r", "ref"])]
    Refile(RefileArgs),
    /// Search tags in Org directory or file
    #[clap(alias = "s")]
    Search(SearchArgs),
    /// A wrapper around the `sed` cli utility to safely manipulate tags
    #[clap()]
    Sed(SedArgs),
    /// Print tags to stdout or to pager
    #[clap(aliases = &["t", "tag"])]
    Tags(TagArgs),
}

// Args
#[derive(Args, Debug, Clone)]
pub struct CountArgs {
    /// Pattern to search for tags
    #[arg(value_name = "PATTERN")]
    pub pattern: Option<String>,
    /// File where to search for tags
    #[arg(long, short, value_name = "FILE", value_hint = ValueHint::FilePath)]
    pub file: Option<String>,
    /// Override config by including files that match <PATTERN>
    #[arg(long, short, value_name = "INCLUDE")]
    pub include: Option<String>,
    /// Override config by excluding files that match <PATTERN>
    #[arg(long, short, value_name = "EXCLUDE")]
    pub exclude: Option<String>,
}

#[derive(Args, Debug, Clone)]
pub struct LocateArgs {
    /// Pattern to search for tags
    #[arg(value_name = "PATTERN")]
    pub pattern: String,
    /// Match the pattern strictly or loosely
    #[arg(short, long, value_name = "STRICT")]
    pub strict: bool,
    /// Override config by including files that match <PATTERN>
    #[arg(long, short, value_name = "INCLUDE")]
    pub include: Option<String>,
    /// Override config by excluding files that match <PATTERN>
    #[arg(long, short, value_name = "EXCLUDE")]
    pub exclude: Option<String>,
}

#[derive(Args, Debug, Clone)]
pub struct RefileArgs {
    /// Pattern to find Org trees to refile
    #[arg(value_name = "PATTERN")]
    pub pattern: String,
    /// Print the contents to stdout instead of pager
    #[arg(short, long, value_name = "NO PAGER")]
    pub no_pager: bool,
    /// Name of the output file. If not given, output is paged or printed to the standard output
    #[arg(value_name = "OUTPUT FILE")]
    pub output_file: Option<String>,
    /// Match the pattern strictly or loosely
    #[arg(short, long, value_name = "STRICT")]
    pub strict: bool,
    /// Override config by including files that match <PATTERN>
    #[arg(long, short, value_name = "INCLUDE")]
    pub include: Option<String>,
    /// Override config by excluding files that match <PATTERN>
    #[arg(long, short, value_name = "EXCLUDE")]
    pub exclude: Option<String>,
}

#[derive(Args, Debug, Clone)]
pub struct SearchArgs {
    /// Pattern used to search for tags
    #[arg(value_name = "PATTERN")]
    pub pattern: String,
    /// File where to search for tags
    #[arg(long, short, value_name = "FILE", value_hint = ValueHint::FilePath)]
    pub file: Option<String>,
    /// Force the output to a pager
    #[arg(long, short, value_name = "PAGER")]
    pub pager: bool,
    /// Override config by including files that match <PATTERN>
    #[arg(long, short, value_name = "INCLUDE")]
    pub include: Option<String>,
    /// Override config by excluding files that match <PATTERN>
    #[arg(long, short, value_name = "EXCLUDE")]
    pub exclude: Option<String>,
}

#[derive(Args, Debug, Clone)]
pub struct SedArgs {
    /// The tag to be substituted.
    /// Note that this command will perform a substitution only on an exact match.
    #[arg(value_name = "TAG")]
    pub tag: String,
    /// The replacement
    #[arg(value_name = "REPLACEMENT")]
    pub replacement: String,
    /// Print additional information about substitutions
    #[arg(long, short, value_name = "VERBOSE")]
    pub verbose: bool,
}

#[derive(Args, Debug, Clone)]
pub struct TagArgs {
    /// Optional file to search instead of searching in the whole Org directory
    #[arg(long, short, value_name = "FILE", value_hint = ValueHint::FilePath)]
    pub file: Option<String>,
    /// Force the output to a pager
    #[arg(long, short, value_name = "PAGER")]
    pub pager: bool,
    /// Print all the tags into a .txt file
    #[arg(long, short = 'P', value_name = "PRINT")]
    pub print: bool,
    // /// Override config by including files that match <PATTERN>
    // pub include: Option<String>,
    // /// Override config by excluding files that match <PATTERN>
    // pub exclude: Option<String>,
    // FIX: find a way to use include/exclude also with this command
}
