use clap::Parser;

#[derive(Parser, Debug)]
#[clap(color = concolor_clap::color_choice())]
struct Args {
    #[clap(flatten)]
    color: concolor_clap::Color,
}

fn main() {
    let args = Args::parse();
    dbg!(&args);
    args.color.apply();
    dbg!(concolor::get(concolor::Stream::Stdout));
    dbg!(concolor::get(concolor::Stream::Stderr));
    dbg!(concolor::get(concolor::Stream::Either));
}
