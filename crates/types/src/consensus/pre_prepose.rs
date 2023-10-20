use crate::on_chain::{Signature, VanillaBundle};

pub struct PrePreposeBundle {
    pub ethereum_height: u64,
    pub bundle:          VanillaBundle,
    /// the signature is over the ethereum height and the bundle hash
    pub signature:       Signature
}
