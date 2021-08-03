use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(setting = concolor_clap::color_choice())]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
struct Args {
    #[structopt(flatten)]
    color: concolor_clap::Color,
}

fn main() {
    let args = Args::from_args();
    dbg!(&args);
    args.color.apply();
    dbg!(concolor_control::get(concolor_control::Stream::Stdout));
    dbg!(concolor_control::get(concolor_control::Stream::Stderr));
    dbg!(concolor_control::get(concolor_control::Stream::Either));
}
