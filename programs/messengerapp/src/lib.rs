use anchor_lang::prelude::*;

declare_id!("HRpsXxxBkx8KTwyr7JZ2Ty5dovpZAWMQE9o3Usn5vDsu");

#[program]
pub mod messengerapp {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, data: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let copy = data.clone();
        base_account.data = data;
        base_account.data_list.push(copy);
        Ok(())
    }
    pub fn update(ctx: Context<Update>, data: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let copy = data.clone();
        base_account.data = data;
        base_account.data_list.push(copy);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    //we have defined 3 accounts here
    #[account(init, payer = user , space = 64+64)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[account]
pub struct BaseAccount {
    pub data: String, //data of type String to store the incoming value of the message
    pub data_list: Vec<String>, //vector of strings to store/persist all the messages in that account.
}
