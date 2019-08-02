use crate::options::Options;

pub trait Format {
    fn format(&self, opt: &Options, indent: usize) -> String;
}
