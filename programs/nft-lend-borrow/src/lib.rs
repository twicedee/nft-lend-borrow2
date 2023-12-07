
pub mod states;
pub mod instructions;
pub mod errors;

pub use states::*;
pub use instructions::*;
pub use errors::ErrorCodes;

declare_id!("AfrDEAwXxkRy2t8Xckoi3CHnawRSEXBEQwxoTW9Rhigy");

#[program]
pub mod nft_lend_borrow {
    use super::*;

    pub fn create_pool(
        ctx: Context<CreatePool>,
        collection_id: Pubkey,
        duration: i64,
    ) -> Result<()> {
        instructions::create_pool::handler(ctx, collection_id, duration)
    }
    pub fn offer_loan(
        ctx: Context<OfferLoan>,
        offer_amount: u64,
    )-> Result<()> {
        instructions::offer_loan::handler(ctx, offer_amount)
    }
    pub fn withdraw_offer(
        ctx: Context<WithdrawOffer>,
        minimum_balance_for_rent_exemption: u64,
    ) ->Result<()> {
        instructions::withdraw_offer::handler(ctx, minimum_balance_for_rent_exemption)
    }
    pub fn borrow(ctx: Context<Borrow>, minimum_balance_for_rent_exemption: u64) -> Result<()> {
        instructions::borrow::handler(ctx, minimum_balance_for_rent_exemption)
    }

    pub fn repay(ctx: Context<Repay>) -> Result<()> {
        instructions::repay::handler(ctx)
    }

    pub fn liquidate(ctx: Context<Liquidate>) -> Result<()> {
        instructions::liquidate::handler(ctx)
    }
}

