use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum ExchangeBoothInstruction {
    InititializeExchangeBooth {
        // TODO
     },
    Deposit {
        // TODO
    },
    Withdraw {
        amount_to_withdraw: u64,
    },
    Exchange {
        // TODO
    },
    CloseExchangeBooth {
        // TODO
    },
}
