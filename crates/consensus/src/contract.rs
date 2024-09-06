use angstrom_types::{consensus::Proposal, contract_payloads::angstrom::{AngstromBundle, TopOfBlockOrder}};

pub fn to_contract_format(proposal: &Proposal) -> AngstromBundle {
    let mut tob_orders = Vec::new();
    for solution in proposal.solutions.iter() {
        // Populate our assets and pairs
        let asset_in_index = 0;
        let asset_out_index = 0;
        // Add the net AMM order to our swaps
        // Add the ToB order to our tob order list - This is currently converting between two ToB order formats
        if let Some(tob) = solution.searcher.as_ref() {
            tob_orders.push(tob.order.clone());
        }
        // 
    }
    AngstromBundle::new(vec![], vec![], vec![], tob_orders, vec![], vec![])
}