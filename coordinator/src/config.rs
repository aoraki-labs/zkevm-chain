use clap::Parser;
use ethers_core::types::Address;
use hyper::Uri;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use std::net::SocketAddr;

#[serde_as]
#[derive(Parser, Serialize, Clone, Debug)]
#[clap(version, about)]
/// zkEVM coordinator, coordinates between the prover and the block production and relays between the bridge contracts in L1 and L2.
pub struct Config {
    #[clap(long, env = "COORDINATOR_RPC_SERVER_NODES")]
    /// Address in the form of host:port of the L2 rpc node(s). Can resolve to multiple addresses.
    pub rpc_server_nodes: String,

    #[clap(long, env = "COORDINATOR_ENABLE_FAUCET")]
    /// Enables faucet to send eth to L1 wallet.
    pub enable_faucet: bool,

    #[clap(long, env = "COORDINATOR_LISTEN")]
    /// Address for the coordinator to listen to, in the format of ip:port.
    pub listen: SocketAddr,

    #[clap(long, env = "COORDINATOR_DUMMY_PROVER")]
    /// Enables dummy prover, so request will not be sent to the actual prover.
    pub dummy_prover: bool,

    #[clap(long, env = "COORDINATOR_MOCK_PROVER", default_value_t = false)]
    /// Only use the mock prover for proof requests.
    pub mock_prover: bool,

    #[clap(long, env = "COORDINATOR_L1_RPC_URL")]
    #[serde_as(as = "DisplayFromStr")]
    /// L1 RPC node URL format.
    pub l1_rpc_url: Uri,

    #[clap(long, env = "COORDINATOR_L1_BRIDGE")]
    /// Ethereum address of the L1 bridge contract.
    pub l1_bridge: Address,

    #[clap(long, env = "COORDINATOR_L1_PRIV")]
    /// Private key for Ethereum L1 wallet.
    pub l1_priv: String,

    #[clap(long, env = "COORDINATOR_L2_RPC_URL")]
    #[serde_as(as = "DisplayFromStr")]
    /// L2 RPC node in http URL format.
    pub l2_rpc_url: Uri,

    #[clap(long, env = "COORDINATOR_PROVER_RPCD_URL")]
    #[serde_as(as = "DisplayFromStr")]
    /// Prover RPC node URL.
    pub prover_rpcd_url: Uri,

    #[clap(long, env = "COORDINATOR_PARAMS_PATH")]
    /// Parameters file or directory to use for the prover requests.
    pub params_path: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self::parse_from(std::env::args().skip(usize::MAX))
    }
}
