use std::collections::HashMap;

use angstrom_types::contract_payloads::Asset;
use reth_primitives::Address;

/// Lets us easily track an array of assets and indexes into that array for
/// contract transformation purposes
pub struct AssetArray {
    assets:     Vec<Asset>,
    assets_idx: HashMap<Address, usize>
}

impl AssetArray {
    pub fn new() -> Self {
        Self { assets: Vec::new(), assets_idx: HashMap::new() }
    }

    pub fn get_asset_addr(&self, idx: usize) -> Address {
        self.assets
            .get(idx)
            .map(|asset| asset.addr)
            .unwrap_or_default()
    }

    pub fn add_or_get_asset_idx(&mut self, asset: Address) -> usize {
        *self.assets_idx.entry(asset).or_insert_with(|| {
            self.assets
                .push(Asset { addr: asset, borrow: 0, save: 0, settle: 0 });
            self.assets.len() - 1
        })
    }

    pub fn get_asset_array(&self) -> Vec<Asset> {
        self.assets.clone()
    }
}

impl Into<Vec<Asset>> for AssetArray {
    fn into(self) -> Vec<Asset> {
        self.assets
    }
}
