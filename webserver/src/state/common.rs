use namada_sdk::tendermint_rpc::HttpClient;

use crate::appstate::AppState;
use crate::service::balance::BalanceService;
use crate::service::chain::ChainService;
use crate::service::governance::GovernanceService;
use crate::service::pos::PosService;
use crate::service::revealed_pk::RevealedPkService;

#[derive(Clone)]
pub struct CommonState {
    pub pos_service: PosService,
    pub gov_service: GovernanceService,
    pub balance_service: BalanceService,
    pub chain_service: ChainService,
    pub revealed_pk_service: RevealedPkService,
    pub client: HttpClient,
}

impl CommonState {
    pub fn new(client: HttpClient, data: AppState) -> Self {
        Self {
            pos_service: PosService::new(data.clone()),
            gov_service: GovernanceService::new(data.clone()),
            balance_service: BalanceService::new(data.clone()),
            chain_service: ChainService::new(data.clone()),
            revealed_pk_service: RevealedPkService::new(data.clone()),
            client,
        }
    }
}
