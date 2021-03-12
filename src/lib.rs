//! near delete guess.k2.testnet k2.testnet
//! near create-account guess.k2.testnet --masterAccount k2.testnet --initialBalance 10
//! cargo build --target wasm32-unknown-unknown --release
//! near deploy --wasmFile target/wasm32-unknown-unknown/release/rust_counter_tutorial.wasm --accountId guess.k2.testnet

use std::cmp::Ordering;
use near_sdk::{Balance, env, near_bindgen, Promise, AccountId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc = near_sdk::wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Guess {
    answer: u8,
    status: bool,
    account_id: AccountId,
    reward: Balance,
    min: u8,
    max: u8,
    count: u8,
}

#[near_bindgen]
impl Guess {
    #[init]
    fn new() -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        Self {
            answer: 0,
            status: false,
            account_id: env::current_account_id(),
            reward: 0,
            min: 0,
            max: 99,
            count: 0,
        }
    }

    /// 支付NEAR生成谜题
    ///
    /// ```bash
    /// near call guess.k2.testnet random --accountId k2.testnet --amount 1
    /// ```
    #[payable]
    pub fn random(&mut self) {
        assert_eq!(false, self.status, "This guess has already begun, please wait for the next round.");
        // TODO 是否需要限制生成谜题时支付的NEAR？
        let amount = env::attached_deposit();
        let random_num: u8 =
            env::random_seed().iter().fold(0_u8, |acc, x| {
                acc.wrapping_add(*x)
            });
        self.answer = random_num % 100;
        self.status = true;
        self.reward = amount;
        self.account_id = env::signer_account_id();
        self.max = 99;
        self.min = 0;
        // env::log(format!("Answer is {}", self.answer).as_bytes());
    }


    /// 查看当前谜题状态
    ///
    /// ```bash
    /// near view guess.k2.testnet get_guess --accountId k2.testnet
    /// ```
    pub fn get_guess(&self) -> (bool, u8, u8, u128) {
        (self.status, self.min, self.max, self.reward)
    }

    /// 支付NEAR猜谜题
    ///
    /// ```bash
    /// near view guess.k2.testnet get_guess --accountId k2.testnet --amount 1
    /// ```
    #[payable]
    pub fn guess(&mut self, answer: u8) {
        assert_eq!(true, self.status, "This turn has not started, waiting for starting.");
        assert!(&answer <= &self.max && &answer >= &self.min, "The magic number is from {} to {}.", &self.min, &self.max);
        // TODO 是否需要限制答题时支付的NEAR？
        let amount = env::attached_deposit();
        let account_id = env::signer_account_id();
        env::log(format!("Guess number is {}", &answer).as_bytes());
        let rst = match &answer.cmp(&self.answer) {
            Ordering::Less => {
                self.min = answer;
                self.wrong_answer(amount);
                String::from("Too small!")
            }
            Ordering::Greater => {
                self.max = answer;
                self.wrong_answer(amount);
                String::from("Too big!")
            }
            Ordering::Equal => {
                let reward = self.reward;
                self.reset();
                // TODO 是否返还全部奖励？
                // TODO 多少个人猜错的情况给出题人奖励？
                Promise::new(account_id.clone()).transfer(reward);
                String::from(format!("You win the reward: {} yoctoNEAR!", reward))
            }
        };
        env::log(rst.as_bytes());
    }

    fn wrong_answer(&mut self, amount: Balance) {
        self.reward = self.reward + amount;
        self.count += 1;
    }

    fn reset(&mut self) {
        self.answer = 0;
        self.status = false;
        self.account_id = env::current_account_id();
        self.reward = 0;
        self.min = 0;
        self.max = 99;
        self.count = 0;
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-counter-tutorial -- --nocapture
 * Note: 'rust-counter-tutorial' comes from cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use near_sdk::{testing_env, VMContext};
    use near_sdk::MockedBlockchain;
    use super::*;

    // part of writing unit tests is setting up a mock context
    // in this example, this is only needed for env::log in the contract
    // this is also a useful list to peek at when wondering what's available in env::*
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }
}
