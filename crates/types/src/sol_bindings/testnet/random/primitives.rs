use rand::Rng;
use reth_primitives::Bytes;

use super::RandomizerSized;

impl<R: Rng + ?Sized> RandomizerSized<Bytes> for R {
    fn gen_sized<const SIZE: usize>(&mut self) -> Bytes {
        let mut val = [0u8; SIZE];
        self.fill_bytes(&mut val);
        val.into()
    }
}
