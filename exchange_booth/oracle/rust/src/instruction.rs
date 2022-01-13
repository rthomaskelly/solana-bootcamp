use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum EchoInstruction {
    /// The contents of the data vector that is provided to the instruction will be copied into the echo_buffer account.
    ///
    /// If the `echo_buffer` account length ( N ) is smaller than the length of data, the instruction will copy the
    /// first N bytes of data into `echo_buffer`.
    ///
    /// If `echo_buffer` has any non-zero data, the instruction will fail.
    ///
    /// Accounts:
    /// | index | writable | signer | description                                  |
    /// |-------|----------|--------|----------------------------------------------|
    /// | 0     | ✅       | ❌     | echo_buffer: Destination account of the data  |
    Echo { data : Vec<u8> },
}
