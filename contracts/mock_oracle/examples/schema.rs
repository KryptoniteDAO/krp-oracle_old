use cosmwasm_schema::write_api;

use mock_oracle::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
        // migrate: MigrateMsg
    }
}
