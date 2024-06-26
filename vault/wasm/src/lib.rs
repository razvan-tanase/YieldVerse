// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                           16
// Async Callback (empty):               1
// Total number of exported functions:  19

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    vault
    (
        init => init
        upgrade => upgrade
        deposit => deposit
        mint => mint
        getAsset => asset
        getBalanceOf => get_balance_of
        getTotalSupply => get_total_supply
        getTotalAssets => total_assets
        getTotalIdle => get_total_idle
        getTotalDebt => get_total_debt
        convertToShares => convert_to_shares
        convertToAssets => convert_to_assets
        maxDeposit => max_deposit
        previewDeposit => preview_deposit
        maxMint => max_mint
        previewMint => preview_mint
        previewWithdraw => preview_withdraw
        previewRedeem => preview_redeem
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
