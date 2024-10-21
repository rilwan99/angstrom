use std::{io::Write, os::unix::process::ExitStatusExt, process::Command};

use convert_case::{Case, Casing};

const CONTRACT_LOCATION: &str = "../../contracts/";
const OUT_DIRECTORY: &str = "../../contracts/out/";
const BINDINGS_PATH: &str = "./src/contract_bindings/mod.rs";

const WANTED_CONTRACTS: [&str; 4] =
    ["Angstrom.sol", "PoolManager.sol", "PoolGate.sol", "MockRewardsManager.sol"];

// builds the contracts crate. then goes and generates bindings on this
fn main() {
    let res = Command::new("forge")
        .arg("build")
        .current_dir(CONTRACT_LOCATION)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    if res.into_raw() != 0 {
        panic!("foundry is not installed on this machine.");
    }

    let sol_macro_invocation = std::fs::read_dir(OUT_DIRECTORY)
        .unwrap()
        .filter_map(|folder| {
            let folder = folder.ok()?;
            let mut path = folder.path();
            let file_name = path.file_name()?.to_str()?;
            if !WANTED_CONTRACTS.contains(&file_name) {
                return None
            }
            let raw = file_name.split('.').collect::<Vec<_>>()[0].to_owned();
            path.push(format!("{raw}.json"));

            Some((raw, path.to_str()?.to_owned()))
        })
        .map(|(name, path_of_contracts)| {
            let mod_name = name.clone().to_case(Case::Snake);
            format!(
                r#"pub mod {mod_name} {{
    alloy::sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        {name},
        "{path_of_contracts}"
    );
}}
"#
            )
        })
        .collect::<Vec<_>>();

    let mut f = std::fs::File::options()
        .write(true)
        .truncate(true)
        .open(BINDINGS_PATH)
        .unwrap();

    for contract_build in sol_macro_invocation {
        writeln!(&mut f, "{}", contract_build).expect("failed to write sol macro to contract");
    }
}
