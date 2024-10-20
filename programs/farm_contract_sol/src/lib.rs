use anchor_lang::prelude::*;

declare_id!("BUrCSwxvLfcqjH9bxfKr2jA58PZRm1Z8XTLEVegp6FWN");

#[program]
mod farm_contract {
    use super::*;
    
    pub fn create_contract(
        ctx: Context<CreateContract>, 
        crop_type: String, 
        agreed_price: u64, 
        quality_score: u64, 
        duration: u64, 
        contract_type: String
    ) -> Result<()> {
        let contract = &mut ctx.accounts.contract;
        contract.farmer = *ctx.accounts.farmer.key;
        contract.buyer = *ctx.accounts.buyer.key;  // Changed to typed account
        contract.crop_type = crop_type;
        contract.agreed_price = agreed_price;
        contract.quality_score = quality_score;
        contract.duration = duration;
        contract.contract_type = contract_type;
        contract.is_fulfilled = false;
        Ok(())
    }

    pub fn fulfill_contract(ctx: Context<FulfillContract>) -> Result<()> {
        let contract = &mut ctx.accounts.contract;
        require!(contract.is_fulfilled == false, CustomError::AlreadyFulfilled);
        contract.is_fulfilled = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateContract<'info> {
    #[account(init, payer = farmer, space = 8 + 32 + 32 + 64 + 64 + 64 + 64 + 8 + 1)]
    pub contract: Account<'info, ContractDetails>,
    #[account(mut)]
    pub farmer: Signer<'info>,
    /// CHECK: This buyer field is a public key representing a user. 
    /// It is expected to be initialized by the program 
    /// and its value is controlled by the caller. 
    pub buyer: Signer<'info>,  // Changed to Signer
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FulfillContract<'info> {
    #[account(mut)]
    pub contract: Account<'info, ContractDetails>,
    pub buyer: Signer<'info>,
}

#[account]
pub struct ContractDetails {
    /// CHECK: This farmer field is a public key representing a user. 
    /// It is safe as it is expected to be initialized by the program 
    /// and its value is controlled by the caller.
    pub farmer: Pubkey,
    
    /// CHECK: This buyer field is a public key representing a user. 
    /// It is safe as it is expected to be initialized by the program 
    /// and its value is controlled by the caller. 
    pub buyer: Pubkey,
    
    /// Maximum length for crop_type
    pub crop_type: String,  // Consider enforcing max length in your logic
    /// Maximum length for contract_type
    pub agreed_price: u64,
    pub quality_score: u64,
    pub duration: u64,
    pub contract_type: String, // Consider enforcing max length in your logic
    pub is_fulfilled: bool,
}

#[error_code]
pub enum CustomError {
    #[msg("Contract is already fulfilled.")]
    AlreadyFulfilled,
}
