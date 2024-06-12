use orm::transactions::{
    InnerTransactionDb, TransactionKindDb, TransactionResultDb,
    WrapperTransactionDb,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TransactionResult {
    Applied,
    Rejected,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TransactionKind {
    TransparentTransfer,
    ShieldedTransfer,
    ShieldingTransfer,
    UnshieldingTransfer,
    Bond,
    Redelegation,
    Unbond,
    Withdraw,
    ClaimRewards,
    VoteProposal,
    InitProposal,
    ChangeMetadata,
    ChangeCommission,
    RevealPk,
    Unknown,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WrapperTransaction {
    pub tx_id: String,
    pub fee_payer: String,
    pub fee_token: String,
    pub gas_limit: String,
    pub block_height: u64,
    pub exit_code: TransactionResult,
    pub atomic: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InnerTransaction {
    pub id: String,
    pub wrapper_id: String,
    pub kind: TransactionKind,
    pub data: Option<String>,
    pub memo: Option<String>,
    pub exit_code: TransactionResult,
}

impl From<TransactionResultDb> for TransactionResult {
    fn from(value: TransactionResultDb) -> Self {
        match value {
            TransactionResultDb::Applied => TransactionResult::Applied,
            TransactionResultDb::Rejected => TransactionResult::Rejected,
        }
    }
}

impl From<TransactionKindDb> for TransactionKind {
    fn from(value: TransactionKindDb) -> Self {
        match value {
            TransactionKindDb::TransparentTransfer => {
                TransactionKind::TransparentTransfer
            }
            TransactionKindDb::ShieldedTransfer => {
                TransactionKind::ShieldedTransfer
            }
            TransactionKindDb::ShieldingTransfer => {
                TransactionKind::ShieldingTransfer
            }
            TransactionKindDb::UnshieldingTransfer => {
                TransactionKind::UnshieldingTransfer
            }
            TransactionKindDb::Bond => TransactionKind::Bond,
            TransactionKindDb::Redelegation => TransactionKind::Redelegation,
            TransactionKindDb::Unbond => TransactionKind::Unbond,
            TransactionKindDb::Withdraw => TransactionKind::Withdraw,
            TransactionKindDb::ClaimRewards => TransactionKind::ClaimRewards,
            TransactionKindDb::VoteProposal => TransactionKind::VoteProposal,
            TransactionKindDb::InitProposal => TransactionKind::InitProposal,
            TransactionKindDb::ChangeMetadata => {
                TransactionKind::ChangeMetadata
            }
            TransactionKindDb::ChangeCommission => {
                TransactionKind::ChangeCommission
            }
            TransactionKindDb::RevealPk => TransactionKind::RevealPk,
            TransactionKindDb::Unknown => TransactionKind::Unknown,
        }
    }
}

impl From<WrapperTransactionDb> for WrapperTransaction {
    fn from(value: WrapperTransactionDb) -> Self {
        Self {
            tx_id: value.id,
            fee_payer: value.fee_payer,
            fee_token: value.fee_token,
            gas_limit: value.gas_limit,
            block_height: value.block_height as u64,
            exit_code: TransactionResult::from(value.exit_code),
            atomic: value.atomic,
        }
    }
}

impl From<InnerTransactionDb> for InnerTransaction {
    fn from(value: InnerTransactionDb) -> Self {
        Self {
            id: value.id,
            wrapper_id: value.wrapper_id,
            kind: TransactionKind::from(value.kind),
            data: value.data,
            memo: value.memo,
            exit_code: TransactionResult::from(value.exit_code),
        }
    }
}
