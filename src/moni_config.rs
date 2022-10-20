use crate::moni::{Moni, MoniPrinter};

pub trait MoniConfig {
    fn to_instance<'a, P: MoniPrinter>(&self) -> Moni<'a, P>;
}
