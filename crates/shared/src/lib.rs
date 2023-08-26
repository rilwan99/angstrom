enum StaleGaurdRequests {
    /// returns the best bundle of the current guard
    BestBundle,
    TxBodies,
    // ? might not need this
    TopOfBundle,
}

enum StaleGaurdRes {
    TxBodies(TransactionBodies),
    BestBundle(Bundle),
    TopOfBundle(TopOfBundle),
}

struct GuardMempool(Vec<Transaction>);

struct TopOfBundle {
    pool_id: u64,
    lp_bribe: usize,
    tx_hash: H256,
    sources: Vec<PubKey>, //relays that have the full searcher tx
}


struct BundleBody {
    pool_id: u64,
    lp_moves: Vec<Transaction>,
    cow_set: Vec<Poolid, CowSet>
    directional: Vec<Transaction>, 
}

struct CowSet {
    cow_tx: Vec<(Transaction, Transaction)>,
}

/// Contains the canonical block bundle of a pool
struct Bundle(Vec<(TopOfBundle, BundleBody)>)
