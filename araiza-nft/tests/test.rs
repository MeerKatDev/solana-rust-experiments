use borsh::BorshDeserialize;
use mpl_token_metadata::state::{Creator, Key};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction, program_option::COption, program_pack::Pack, pubkey::Pubkey,
    signature::Keypair, signer::Signer, signers::Signers, transaction::Transaction,
};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account_idempotent,
};
use spl_token::state::{Account, AccountState, Mint};

type Error = Box<dyn std::error::Error>;

#[test]
fn test() {
    let client = RpcClient::new("https://api.testnet.solana.com");
    let authority = solana_sdk::signature::read_keypair_file("./authority.keypair").unwrap();
    let user = solana_sdk::signature::read_keypair_file("./user.keypair").unwrap();

    let mint_address = dbg!(create_mint(&client, &authority, 0).unwrap());
    let mint = client.get_account(&mint_address).unwrap();
    let mint_data = Mint::unpack(&mint.data).unwrap();
    assert_eq!(mint.owner, spl_token::ID);
    assert_eq!(mint_data.mint_authority, COption::Some(authority.pubkey()));
    assert_eq!(mint_data.supply, 0);
    assert_eq!(mint_data.decimals, 0);
    assert_eq!(mint_data.freeze_authority, COption::None);

    let account_address =
        create_associated_account(&client, &authority, &mint_address, &user.pubkey()).unwrap();
    let account = client.get_account(&account_address).unwrap();
    let account_data = Account::unpack(&account.data).unwrap();
    assert_eq!(account.owner, spl_token::ID);
    assert_eq!(account_data.mint, mint_address);
    assert_eq!(account_data.owner, user.pubkey());
    assert_eq!(account_data.amount, 0);
    assert_eq!(account_data.delegate, COption::None);
    assert_eq!(account_data.state, AccountState::Initialized);
    assert_eq!(account_data.is_native, COption::None);
    assert_eq!(account_data.delegated_amount, 0);
    assert_eq!(account_data.close_authority, COption::None);

    mint_token(&client, &authority, &mint_address, &account_address, 1).unwrap();
    let account = client.get_account(&account_address).unwrap();
    let account_data = Account::unpack(&account.data).unwrap();
    assert_eq!(account.owner, spl_token::ID);
    assert_eq!(account_data.mint, mint_address);
    assert_eq!(account_data.owner, user.pubkey());
    assert_eq!(account_data.amount, 1);
    assert_eq!(account_data.delegate, COption::None);
    assert_eq!(account_data.state, AccountState::Initialized);
    assert_eq!(account_data.is_native, COption::None);
    assert_eq!(account_data.delegated_amount, 0);
    assert_eq!(account_data.close_authority, COption::None);

    let metadata_address = create_metadata(&client, &authority, &mint_address, Metadata {
        name: "Mlabs Gold Star".to_string(),
        uri: "https://upload.wikimedia.org/wikipedia/commons/thumb/2/29/Gold_Star.svg/1024px-Gold_Star.svg.png".to_string(),
        symbol: "★".into(),
        creators: None,
        seller_fee_basis_points: 0,
    }).unwrap();
    let metadata = client.get_account(&metadata_address).unwrap();
    let metadata_data =
        mpl_token_metadata::state::Metadata::deserialize(&mut metadata.data.as_slice()).unwrap();
    assert_eq!(metadata_data.mint, mint_address);
    assert!(metadata_data.data.name.starts_with("Mlabs Gold Star"));

    let master_edition_address =
        create_master_edition(&client, &authority, &mint_address, &metadata_address).unwrap();
    let master_edition = client.get_account(&master_edition_address).unwrap();
    let master_edition_data = mpl_token_metadata::state::MasterEditionV2::deserialize(
        &mut master_edition.data.as_slice(),
    )
    .unwrap();
    assert_eq!(master_edition_data.key, Key::MasterEditionV2);
    assert_eq!(master_edition_data.max_supply, Some(0));
    assert_eq!(master_edition_data.supply, 0);

    assert!(mint_token(&client, &authority, &mint_address, &account_address, 1).is_err());
}

fn create_mint(client: &RpcClient, authority: &Keypair, decimals: u8) -> Result<Pubkey, Error> {
    let mint = Keypair::new();
    let space = spl_token::state::Mint::LEN;
    let lamports = client.get_minimum_balance_for_rent_exemption(space)?;
    let create = solana_sdk::system_instruction::create_account(
        &authority.pubkey(),
        &mint.pubkey(),
        lamports,
        space as u64,
        &spl_token::ID,
    );
    let initialize = spl_token::instruction::initialize_mint(
        &spl_token::ID,
        &mint.pubkey(),
        &authority.pubkey(),
        None,
        decimals,
    )?;
    execute(client, authority, &[create, initialize], [authority, &mint])?;
    Ok(mint.pubkey())
}

fn create_associated_account(
    client: &RpcClient,
    payer: &Keypair,
    mint: &Pubkey,
    owner: &Pubkey,
) -> Result<Pubkey, Error> {
    let address = get_associated_token_address(owner, mint);
    let instruction =
        create_associated_token_account_idempotent(&payer.pubkey(), owner, mint, &spl_token::ID);
    execute(client, payer, &[instruction], [payer])?;
    Ok(address)
}

fn mint_token(
    client: &RpcClient,
    authority: &Keypair,
    mint: &Pubkey,
    account: &Pubkey,
    amount: u64,
) -> Result<(), Error> {
    let instruction = spl_token::instruction::mint_to(
        &spl_token::ID,
        mint,
        account,
        &authority.pubkey(),
        &[&authority.pubkey()],
        amount,
    )?;
    execute(client, authority, &[instruction], [authority])?;
    Ok(())
}

struct Metadata {
    name: String,
    uri: String,
    symbol: String,
    creators: Option<Vec<Creator>>,
    seller_fee_basis_points: u16,
}

fn create_metadata(
    client: &RpcClient,
    authority: &Keypair,
    mint: &Pubkey,
    metadata: Metadata,
) -> Result<Pubkey, Error> {
    let metadata_account = associated_metaplex_token_address(&mpl_token_metadata::id(), mint);
    let instruction = mpl_token_metadata::instruction::create_metadata_accounts(
        mpl_token_metadata::id(),
        metadata_account,
        *mint,
        authority.pubkey(),
        authority.pubkey(),
        authority.pubkey(),
        metadata.name,
        metadata.symbol,
        metadata.uri,
        metadata.creators,
        metadata.seller_fee_basis_points,
        true,
        false,
    );
    execute(client, authority, &[instruction], [authority])?;
    Ok(metadata_account)
}

fn associated_metaplex_token_address(wallet: &Pubkey, mint: &Pubkey) -> Pubkey {
    let seeds = [
        mpl_token_metadata::state::PREFIX.as_bytes(),
        wallet.as_ref(),
        mint.as_ref(),
    ];
    let (account, _) = Pubkey::find_program_address(&seeds, &mpl_token_metadata::id());
    account
}

fn create_master_edition(
    client: &RpcClient,
    authority: &Keypair,
    mint: &Pubkey,
    metadata: &Pubkey,
) -> Result<Pubkey, Error> {
    let master_address = associated_metaplex_edition_address(mint, b"edition");
    let instruction = mpl_token_metadata::instruction::create_master_edition(
        mpl_token_metadata::id(),
        master_address,
        *mint,
        authority.pubkey(),
        authority.pubkey(),
        *metadata,
        authority.pubkey(),
        Some(0),
    );
    execute(client, authority, &[instruction], [authority])?;
    Ok(master_address)
}

fn associated_metaplex_edition_address(mint: &Pubkey, edition: &[u8]) -> Pubkey {
    let program_id = mpl_token_metadata::id();
    let seeds = [b"metadata", program_id.as_ref(), mint.as_ref(), edition];
    let (account, _) = Pubkey::find_program_address(&seeds, &mpl_token_metadata::id());
    account
}

fn execute<T: Signers>(
    client: &RpcClient,
    payer: &Keypair,
    instructions: &[Instruction],
    signers: T,
) -> Result<(), Error> {
    let blockhash = client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        instructions,
        Some(&payer.pubkey()),
        &signers,
        blockhash,
    );
    client.send_and_confirm_transaction(&transaction)?;
    Ok(())
}
