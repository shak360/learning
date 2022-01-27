use color_eyre::eyre::Result;
use structopt::StructOpt;

/// A CLI for the growing and curation of a digital garden.
#[derive(StructOpt, Debug)]
#[structopt(name = "garden")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Write something in your garden.
    ///
    /// This command will open your $EDITOR, wait for you
    /// to write something, and then save the file to your
    ///  garden.
    Write {
        #[structopt(short, long)]
        /// Optionally set a title
        title: Option<String>,
    },
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let opt = Opt::from_args();
    dbg!(opt);
    todo!();
}
