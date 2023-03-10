use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("nft");

    blockchain.register_contract_builder("file:output/nft.wasm", ping_pong::ContractBuilder);
    blockchain
}

#[test]
fn ping_pong_call_ping_rs() {
    elrond_wasm_debug::mandos_rs("mandos/nft-call-ping.scen.json", world());
}

#[test]
fn ping_pong_call_ping_second_user_rs() {
    elrond_wasm_debug::mandos_rs("mandos/nft-call-ping-second-user.scen.json", world());
}

#[test]
fn ping_pong_call_ping_twice_rs() {
    elrond_wasm_debug::mandos_rs("mandos/nft-call-ping-twice.scen.json", world());
}

#[test]
fn ping_pong_call_ping_wrong_amount_rs() {
    elrond_wasm_debug::mandos_rs("mandos/nft-call-ping-wrong-amount.scen.json", world());
}

#[test]
fn ping_pong_call_pong_rs() {
    elrond_wasm_debug::mandos_rs("mandos/nft-call-pong.scen.json", world());
}

#[test]
fn ping_pong_call_pong_before_deadline_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/nft-call-pong-before-deadline.scen.json",
        world(),
    );
}

#[test]
fn ping_pong_call_pong_twice_rs() {
    elrond_wasm_debug::mandos_rs("mandos/nft-call-pong-twice.scen.json", world());
}

#[test]
fn ping_pong_call_pong_without_ping_rs() {
    elrond_wasm_debug::mandos_rs("mandos/nft-call-pong-without-ping.scen.json", world());
}

#[test]
fn ping_pong_init_rs() {
    elrond_wasm_debug::mandos_rs("mandos/nft-init.scen.json", world());
}
