//! Program instruction processor

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};

/// Amount of bytes of account data to allocate
pub const SIZE: usize = 42;

/// Instruction processor
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let program_id = next_account_info(account_info_iter)?;
    let nft_owner = next_account_info(account_info_iter)?;
    /// Main sol
    let nft_holder_owner_account = next_account_info(account_info_iter)?;
    // let temp_nft_account = next_account_info(account_info_iter)?;
    let nft_program_id = next_account_info(account_info_iter)?;

    let fractioned_nft_program_id = next_account_info(account_info_iter)?;
    let fractioned_nft_info_account = next_account_info(account_info_iter)?;
    let nft_fraction_account = next_account_info(account_info_iter)?;

    if !nft_owner.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (pda, _nonce) =
        Pubkey::find_program_address(&[&nft_program_id.pubkey().to_bytes()[..32]], program_id);

    let owner_change_ix = spl_token::instruction::set_authority(
        nft_program_id.key,
        nft_holder_owner_account.key,
        Some(&pda),
        spl_token::instruction::AuthorityType::AccountOwner,
        nft_owner.key,
        &[&nft_owner.key],
    )?;

    // Mint Frantionlised NFTs to nft_account
    let mut fraction_info = NFT::unpack_unchecked(&fractioned_nft_info_account.data.borrow())?;

    if fraction_info.is_initialized() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }
    fraction_info.is_initialized = true;
    fraction_info.initializer_pubkey = *initializer.key;
    fraction_info.temp_token_account_pubkey = *temp_token_account.key;
    fraction_info.initializer_token_to_receive_account_pubkey = *token_to_receive_account.key;
    fraction_info.expected_amount = amount;
    NFT::pack(
        fraction_info,
        &mut fractioned_nft_info_account.data.borrow_mut(),
    )?;
    let (pda, _nonce) = Pubkey::find_program_address(&[b"nft"], program_id);

    let ix = spl_token::instruction::mint(
        fractioned_nft_program_id.key,
        nft_fraction_account.key,
        nft_owner.key,
        nft_owner.key,
        &[],
        100,
    )?;

    msg!("Calling the token program to transfer token account ownership...");
    invoke(
        &owner_change_ix,
        &[
            nft_holder_owner_account.clone(),
            nft_owner.clone(),
            nft_program_id.clone(),
        ],
    )?;

    // let instruction = spl_token::instruction::transfer(token_program_id: &Pubkey, source_pubkey: &Pubkey, destination_pubkey: &Pubkey, authority_pubkey: &Pubkey, signer_pubkeys: &[&Pubkey], amount: u64)
    // invoke(&instruction, accounts)?;

    // // Create in iterator to safety reference accounts in the slice
    // let account_info_iter = &mut accounts.iter();
    // msg!("{:?}", account_info_iter);
    // // Account info for the program being invoked
    // let system_program_info = next_account_info(account_info_iter)?;
    // // Account info to allocate
    // let allocated_info = next_account_info(account_info_iter)?;

    // let expected_allocated_key =
    //     Pubkey::create_program_address(&[b"You pass butter", &[instruction_data[0]]], program_id)?;
    // if *allocated_info.key != expected_allocated_key {
    //     // allocated key does not match the derived address
    //     return Err(ProgramError::InvalidArgument);
    // }

    // // Invoke the system program to allocate account data
    // invoke_signed(
    //     &system_instruction::allocate(allocated_info.key, SIZE as u64),
    //     // Order doesn't matter and this slice could include all the accounts and be:
    //     // `&accounts`
    //     &[
    //         system_program_info.clone(), // program being invoked also needs to be included
    //         allocated_info.clone(),
    //     ],
    //     &[&[b"You pass butter", &[instruction_data[0]]]],
    // )?;

    Ok(())
}
