use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use crux_core::type_generation::facet::{Config, TypeRegistry};
use log::info;

use shared::EggTimer;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Language {
    Typescript,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_enum)]
    language: Language,
    #[arg(short, long)]
    output_dir: PathBuf,
}

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    let args = Args::parse();

    let typegen_app = TypeRegistry::new().register_app::<EggTimer>()?.build()?;

    match args.language {
        Language::Typescript => {
            info!("Typegen for TypeScript");
            typegen_app.typescript(
                &Config::builder("app", &args.output_dir)
                    .add_extensions()
                    .add_runtimes()
                    .build(),
            )?;
        }
    }

    Ok(())
}
