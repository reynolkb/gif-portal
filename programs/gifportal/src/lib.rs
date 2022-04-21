// use declaration, similar to import statement
use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

// program id
// has information for solana on how to run our program
// solana can see all the accounts generated by the program and easily reference them
declare_id!("3gyBddu56VrbYSncpFQHUGd7155e2HjWcdqf5bfkxHcS");

// everything in the section below is our program
// # are macros which attach code to our module, similar to inheriting a clause in javascript
#[program]
// this is our program. allows us to call our program from the frontend. stateless because they don't hold data. program must interact with account (files that store data)
// tells us this is a rust module, similar to clause in javascript
pub mod gifportal {
    use super::*;

    // public function
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let item = ItemStruct {
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key
        };

        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }
}

// another macro
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer=user, space=9000)]
    pub base_account: Account<'info, BaseAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    // system program creates accounts on solana
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct AddGif<'info> {
    // mut because we need to change the total Gifs in our base account
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>
}

// tells anchor how to serilaize and deserialize the struct
// data is being stored in account, that account is a file
// we serialize the data that it's storing into a binary format before we store it
// when we want to retireve the data, we deserialize the data before we can retrieve it
// this line takes care of that, to make sure our data is properly serialized and deserialized
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]

pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey
}

// tells our program what type of account it can make and what it can hold inside of it
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    // vector
    pub gif_list: Vec<ItemStruct>
}