#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

#[derive(Debug, PartialEq)]
enum Rounding {
    RoundDown,
    RoundUp,
}

#[multiversx_sc::contract]
pub trait Vault {
    #[init]
    fn init(&self, asset: EgldOrEsdtTokenIdentifier) {
        require!(asset.is_valid(), "Invalid ESDT token identifier.");
        self.asset_token_identifier().set(asset);
    }

    #[upgrade]
    fn upgrade(&self) {}

    // ========== ENDPOINTS ===========

    #[endpoint(deposit)]
    #[payable("*")]
    fn deposit(&self) -> BigUint {
        let (token, _, assets) = self.call_value().egld_or_single_esdt().into_tuple();
        let recipient = self.blockchain().get_caller();

        require!(assets <= self._max_deposit(recipient.clone()), "Exceed deposit limit");
        require!(token == self.asset(), "Wrong token");

        // Record the change in total assets.
        self.total_idle().update(|total_idle| {
            *total_idle += assets.clone();
        });

        // Issue the corresponding shares for assets.
        let shares = self.issue_shares_for_amount(assets, recipient);
        require!(shares > 0, "Cannot mint 0.");

        // event deposit

        shares
    }

    #[endpoint(mint)]
    #[payable("*")]
    fn mint(&self, shares: BigUint) -> BigUint {
        let recipient = self.blockchain().get_caller();

        // Convert shares to assets.
        let assets = self._convert_to_assets(shares.clone(), Rounding::RoundUp);

        require!(assets > 0, "Cannot deposit 0");
        require!(assets <= self._max_deposit(recipient), "Exceed deposit limit");

        // let (token, _, payment) = self.call_value().egld_or_single_esdt().into_tuple();
        // require!(token == self.asset(), "Wrong token");

        // // Record the change in total assets.
        // self.total_idle().update(|total_idle| {
        //     *total_idle += assets.clone();
        // });

        // // Issue the corresponding shares for assets.
        // self.issue_shares(shares.clone(), recipient);

        // event Mint(recipient, shares, assets);

        assets
    }

    // ========== INTERNALS ===========

    fn issue_shares_for_amount(&self, amount: BigUint, recipient: ManagedAddress) -> BigUint {
        let total_assets = self.total_assets();
        let total_supply = self.get_total_supply();

        // If no supply PPS = 1.
        let new_shares = if total_supply == 0 {
            amount
        } else {
            amount.clone() * total_supply / (total_assets - amount)
        };

        // We don't make the function revert
        if new_shares == 0 {
            BigUint::zero();
        }

        self.issue_shares(new_shares.clone(), recipient);

        new_shares
    }

    fn issue_shares(&self, shares: BigUint, recipient: ManagedAddress) {
        self.total_supply().update(|total_supply| {
            *total_supply += shares.clone();
        });

        self.balance_of(recipient).update(|balance| {
            *balance += shares;
        });

        // event Transfer(address(0), recipient, shares);
    }

    fn _convert_to_shares(&self, amount: BigUint, rounding: Rounding) -> BigUint {
        let total_assets = self.total_assets();
        let total_supply = self.get_total_supply();

        if total_assets == 0 {
            // if total_assets and total_supply is 0, price_per_share is 1
            if total_supply == 0 {
                return amount;
            } else {
                // Else if total_supply > 0 price_per_share is 0
                BigUint::zero();
            }
        }

        let numerator = amount * total_supply;
        let mut shares = numerator.clone() / total_assets.clone();

        if rounding == Rounding::RoundUp && numerator % total_assets != 0 {
            shares += 1u32;
        }

        shares
    }

    fn _convert_to_assets(&self, shares: BigUint, rounding: Rounding) -> BigUint {
        let total_supply = self.get_total_supply();

        // if total_supply is 0, price_per_share is 1
        if total_supply == 0 {
            return shares;
        }

        let numerator = shares * self.total_assets();
        let mut amount = numerator.clone() / total_supply.clone();

        if rounding == Rounding::RoundUp && numerator % total_supply != 0 {
            amount += 1u32;
        }

        amount
    }

    fn _max_deposit(&self, receiver: ManagedAddress) -> BigUint {
        let max_deposit = BigUint::from(1000u32);
        let balance = self.get_balance_of(receiver);

        max_deposit - balance
    }

    // ========== VIEWS ===========

    #[view(getAsset)]
    fn asset(&self) -> EgldOrEsdtTokenIdentifier {
        self.asset_token_identifier().get()
    }

    #[view(getBalanceOf)]
    fn get_balance_of(&self, address: ManagedAddress) -> BigUint {
        self.balance_of(address).get()
    }

    #[view(getTotalSupply)]
    fn get_total_supply(&self) -> BigUint {
        self.total_supply().get()
    }

    #[view(getTotalAssets)]
    fn total_assets(&self) -> BigUint {
        self.get_total_idle() + self.get_total_debt()
    }

    #[view(getTotalIdle)]
    fn get_total_idle(&self) -> BigUint {
        self.total_idle().get()
    }

    #[view(getTotalDebt)]
    fn get_total_debt(&self) -> BigUint {
        self.total_debt().get()
    }

    #[view(convertToShares)]
    fn convert_to_shares(&self, assets: BigUint) -> BigUint {
        self._convert_to_shares(assets, Rounding::RoundDown)
    }

    #[view(convertToAssets)]
    fn convert_to_assets(&self, shares: BigUint) -> BigUint {
        self._convert_to_assets(shares, Rounding::RoundDown)
    }

    #[view(maxDeposit)]
    fn max_deposit(&self) -> BigUint {
        let receiver = self.blockchain().get_caller();
        self._max_deposit(receiver)
    }

    #[view(previewDeposit)]
    fn preview_deposit(&self, assets: BigUint) -> BigUint {
        self._convert_to_shares(assets, Rounding::RoundDown)
    }

    #[view(maxMint)]
    fn max_mint(&self) -> BigUint {
        let receiver = self.blockchain().get_caller();
        let max_deposit = self._max_deposit(receiver);
        self._convert_to_shares(max_deposit, Rounding::RoundDown)
    }

    #[view(previewMint)]
    fn preview_mint(&self, shares: BigUint) -> BigUint {
        self._convert_to_assets(shares, Rounding::RoundUp)
    }

    #[view(previewWithdraw)]
    fn preview_withdraw(&self, assets: BigUint) -> BigUint {
        self._convert_to_shares(assets, Rounding::RoundUp)
    }

    #[view(previewRedeem)]
    fn preview_redeem(&self, shares: BigUint) -> BigUint {
        self._convert_to_assets(shares, Rounding::RoundDown)
    }

    // ========== STORAGE ==========
    
    // Underlying token used by the vault.
    #[storage_mapper("assetTokenIdentifier")]
    fn asset_token_identifier(&self) -> SingleValueMapper<EgldOrEsdtTokenIdentifier>;

    // Current assets held in the vault contract.
    #[storage_mapper("totalIdle")]
    fn total_idle(&self) -> SingleValueMapper<BigUint>;

    // Total amount of assets that has been deposited in strategies.
    #[storage_mapper("totalDebt")]
    fn total_debt(&self) -> SingleValueMapper<BigUint>;

    // Total amount of shares that are currently minted.
    #[storage_mapper("totalSupply")]
    fn total_supply(&self) -> SingleValueMapper<BigUint>;

    // Amount of shares per account
    #[storage_mapper("balanceOf")]
    fn balance_of(&self, address: ManagedAddress) -> SingleValueMapper<BigUint>;
}
