use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract("mxsc:output/vault.mxsc.json", vault::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    world().run("scenarios/vault.scen.json");
}
