use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Oracle {
    #[serde(with = "u64_dec_format")]
    pub last_report: Timestamp,
    pub price_reports: u64,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum VOracle {
    Current(Oracle),
}

impl From<VOracle> for Oracle {
    fn from(v: VOracle) -> Self {
        match v {
            VOracle::Current(c) => c,
        }
    }
}

impl From<Oracle> for VOracle {
    fn from(c: Oracle) -> Self {
        VOracle::Current(c)
    }
}

impl Oracle {
    pub fn new() -> Self {
        Self {
            last_report: 0,
            price_reports: 0,
        }
    }
}

impl Contract {
    pub fn internal_get_oracle(&self, account_id: &AccountId) -> Option<Oracle> {
        self.oracles.get(account_id).map(|o| o.into())
    }

    pub fn internal_set_oracle(&mut self, account_id: &AccountId, oracle: Oracle) {
        self.oracles.insert(account_id, &oracle.into());
    }
}
