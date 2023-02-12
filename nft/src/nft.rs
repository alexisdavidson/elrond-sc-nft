#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait NftMinter {
    #[init]
    fn init(&self) {

    }

    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::redundant_closure)]
    #[only_owner]
    #[endpoint(createNft)]
    fn create_nft(
        &self
    ) {
    }

}
