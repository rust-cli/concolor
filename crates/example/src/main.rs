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
    dbg!(concolor_control::get(concolor_control::Stream::Stdout));
    dbg!(concolor_control::get(concolor_control::Stream::Stderr));
    dbg!(concolor_control::get(concolor_control::Stream::Either));
}
