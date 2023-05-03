use anchor_lang::prelude::*;
use notes::cpi::{accounts::WriteNote, write_note};
use notes::program::Notes;

declare_id!("Ev9dZC4VW8XJok85dEz3Er6psN2DdJkuSbYVzs8FGg8W");

#[program]
pub mod note_taker {
    use super::*;

    pub fn take_note(ctx: Context<TakeNote>, note: String) -> Result<()> {
        msg!("Making CPI to notes program...");

        write_note(ctx.accounts.take_note_ctx(), note)?;

        msg!("CPI finished!");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct TakeNote<'info> {
    #[account(mut)]
    pub note: Signer<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub notes_program: Program<'info, Notes>,
}

impl<'info> TakeNote<'info> {
    pub fn take_note_ctx(&self) -> CpiContext<'_, '_, '_, 'info, WriteNote<'info>> {
        let cpi_program = self.notes_program.to_account_info();
        let cpi_accounts = WriteNote {
            authority: self.authority.to_account_info(),
            note: self.note.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }
}
