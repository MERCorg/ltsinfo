use std::error::Error;
use std::fs::File;
use std::io::stdout;
use std::io::BufWriter;
use std::process::ExitCode;

use clap::Parser;
use clap::ValueEnum;

use ltsinfo_io::io_aut::read_aut;
use ltsinfo_io::io_aut::write_aut;
use ltsinfo_reduction::branching_bisim_sigref;
use ltsinfo_reduction::quotient_lts_efficient;
use ltsinfo_reduction::BlockPartition;
use ltsinfo_reduction::IncomingTransitions;
use ltsinfo_utilities::Timing;
use ltsinfo_reduction::preprocess_branching;

#[cfg(feature = "measure-allocs")]
#[global_allocator]
static MEASURE_ALLOC: unsafety::AllocCounter = unsafety::AllocCounter;

#[cfg(feature = "measure-allocs")]
use log::info;

#[cfg(not(target_env = "msvc"))]
#[cfg(not(feature = "measure-allocs"))]
#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[derive(Clone, Debug, ValueEnum)]
enum Equivalence {
    StrongBisim,
    StrongBisimNaive,
    BranchingBisim,
    BranchingBisimNaive,
}

#[derive(clap::Parser, Debug)]
#[command(name = "anonymous", about = "A command line rewriting tool")]
struct Cli {
    equivalence: Equivalence,

    filename: String,

    output: Option<String>,

    #[arg(short, long)]
    tau: Option<Vec<String>>,

    #[arg(long)]
    time: bool,
}

fn main() -> Result<ExitCode, Box<dyn Error>> {
    env_logger::init();

    let cli = Cli::parse();

    let file = File::open(cli.filename)?;

    let mut timing = Timing::new();
    let mut timepre: ltsinfo_utilities::Timer = timing.start("preprocess");

    let preprocessed_lts = {
        let lts: ltsinfo_lts::LabelledTransitionSystem = read_aut(&file, cli.tau.unwrap_or_default())?;
        let preproccessed_lts = preprocess_branching(&lts);
        preproccessed_lts
    }; // lts is dropped here

    let incoming = IncomingTransitions::new(&preprocessed_lts);
    timepre.finish();

    let partition: BlockPartition = match cli.equivalence {
        Equivalence::StrongBisim => branching_bisim_sigref(&preprocessed_lts, &incoming, &mut timing),
        Equivalence::StrongBisimNaive => branching_bisim_sigref(&preprocessed_lts, &incoming, &mut timing),
        Equivalence::BranchingBisim => branching_bisim_sigref(&preprocessed_lts, &incoming, &mut timing),
        Equivalence::BranchingBisimNaive => branching_bisim_sigref(&preprocessed_lts, &incoming, &mut timing),
    };

    let mut quotient_time = timing.start("quotient");
    let quotient_lts = quotient_lts_efficient(
        &preprocessed_lts,
        &partition);

    if let Some(file) = cli.output {
        let mut writer = BufWriter::new(File::create(file)?);
        write_aut(&mut writer, &quotient_lts)?;
    } else {
        write_aut(&mut stdout(), &quotient_lts)?;
    }
    quotient_time.finish();

    if cli.time {
        timing.print();
    }

    #[cfg(feature = "measure-allocs")]
    eprintln!("allocations: {}", MEASURE_ALLOC.number_of_allocations());

    Ok(ExitCode::SUCCESS)
}
