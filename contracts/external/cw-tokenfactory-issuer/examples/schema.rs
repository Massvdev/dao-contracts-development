use cosmwasm_schema::write_api;

use cw_tokenfactory_issuer::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, SudoMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
        sudo: SudoMsg,
    }
}
