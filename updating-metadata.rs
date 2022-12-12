use solana_sdk::{
    account::Account,
    entrypoint,
    clock::Slot,
    instruction::InstructionError,
    pubkey::Pubkey,
};

struct NFTMetadata {
    metadata: Vec<u8>,
}

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[Account],
    instruction_data: &[u8],
) -> Result<(), InstructionError> {
    
    if instruction_data[0] != 1 {
        return Ok(());
    }

    let (nft_id, new_metadata) = decode_instruction_data(instruction_data)?;

    // Retrieve the account for the NFT
    let nft_account = get_account(&accounts, &nft_id)?;

    // Decode the metadata for the NFT
    let mut nft_metadata = decode_account_data(&nft_account)?;

    // Update the metadata for the NFT
    nft_metadata.metadata = new_metadata;

    // Encode the updated metadata and write it back to the NFT account
    let updated_account_data = encode_account_data(&nft_metadata)?;
    nft_account.data = updated_account_data;
    Ok(())
}


fn decode_instruction_data(
    instruction_data: &[u8],
) -> Result<(Pubkey, Vec<u8>), InstructionError> {
    if instruction_data.len() < 33 {
        return Err(InstructionError::InvalidInstructionData);
    }

    let nft_id = Pubkey::new(&instruction_data[0..32]);

    let new_metadata = instruction_data[32..].to_vec();

    Ok((nft_id, new_metadata))
}


fn get_account(
    accounts: &[Account],
    nft_id: &Pubkey,
) -> Result<&Account, InstructionError> {
    for account in accounts {
        if account.id == *nft_id {
            return Ok(account);
        }
    }

    Err(InstructionError::AccountNotFound)
}
