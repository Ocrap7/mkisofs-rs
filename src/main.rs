use structopt::StructOpt;

use mkisofs::iso::option::Opt;

fn main() {
    let mut opt = Opt::from_args();
    mkisofs::iso::create_iso(&mut opt).unwrap();
}
