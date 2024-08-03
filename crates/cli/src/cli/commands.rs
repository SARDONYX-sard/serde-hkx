use super::convert::Convert;

#[derive(Debug, clap::Parser)]
#[clap(version, about)]
pub(crate) enum Commands {
    /// Convert hkx <-> xml
    #[clap(arg_required_else_help = true)]
    Convert(Convert),
}
