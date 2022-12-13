use crate::polkadot::runtime_types::pallet_conviction_voting::vote::Voting::{
    Casting, Delegating, __Ignore,
};
use std::str::FromStr;
use structopt::StructOpt;
use subxt::{ext::sp_runtime::AccountId32, OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "kusama_metadata.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let args = Cli::from_args();

    let dest = AccountId32::from_str(&args.address.as_str()).unwrap();

    let api = OnlineClient::<PolkadotConfig>::from_url("wss://kusama-rpc.dwellir.com:443").await?;

    let consts = polkadot::constants().referenda().tracks();

    let tracks = api.constants().at(&consts).unwrap();

    for track in tracks.iter() {
        println!("=======================");
        println!("| AccountId: \t{}", &args.address);
        println!("| Track ID: \t{}", track.0);
        println!("| Track Name: \t{}\n|", track.1.name);
        let voting_address = polkadot::storage()
            .conviction_voting()
            .voting_for(&dest, track.0);

        let voting_for_info = api.storage().fetch(&voting_address, None).await?;

        match voting_for_info {
            Some(voting) => match voting {
                Casting(casting) => {
                    let delegations = casting.delegations;
                    let votes = (*&delegations.votes as f64) / 1_000_000_000_000u64 as f64;
                    let capital = (*&delegations.capital as f64) / 1_000_000_000_000u64 as f64;

                    println!("| Votes: \t{}", votes);
                    println!("| Capital: \t{} KSM", capital);
                }
                Delegating(_) => todo!(),
                __Ignore(_) => {}
            },
            None => println!(
                "No delegations votes for this track {trackId}",
                trackId = track.0
            ),
        }
        println!("=======================\n");
    }

    Ok(())
}

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "a")]
    address: String,
}
