#[cfg(test)]
mod tests {
    use crate::contract::{execute, instantiate};
    use crate::error::ContractError;
    use crate::msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, PythFeederConfigResponse};
    use crate::querier::{query_config, query_pyth_feeder_config};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::Addr;
    use pyth_sdk_cw::PriceIdentifier;

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        let pyth_contract = "pyth_contract_address";

        let _msg = InstantiateMsg {
            pyth_contract: pyth_contract.to_string(),
            owner: Addr::unchecked("creator"),
        };
        let _info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(0, _res.messages.len());

        // Verify the config is stored correctly
        assert_eq!(
            query_config(deps.as_ref()).unwrap(),
            ConfigResponse {
                owner: "creator".to_string(),
                pyth_contract: pyth_contract.to_string(),
            }
        );
    }

    #[test]
    fn test_config_feed_info() {
        let mut deps = mock_dependencies();
        let pyth_contract = "pyth_contract_address";

        let _msg = InstantiateMsg {
            pyth_contract: pyth_contract.to_string(),
            owner: Addr::unchecked("creator"),
        };
        let _info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), _info, _msg).unwrap();
        assert_eq!(0, _res.messages.len());

        // Negative test case with insufficient permissions
        let _msg = ExecuteMsg::ConfigFeedInfo {
            asset: "usei".to_string(),
            price_feed_id: "ff61491a931112ddf1bd8147cd1b641375f79f5825126d665480874634fd0acd"
                .to_string(),
            price_feed_symbol: "Crypto.ETH/USD".to_string(),
            price_feed_decimal: 8,
            check_feed_age: true,
            price_feed_age: 60,
        };
        let _info = mock_info("random_user", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg.clone());
        match _res {
            Err(ContractError::Unauthorized { .. }) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // Positive test case
        let _info = mock_info("creator", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg.clone()).unwrap();
        assert_eq!(0, _res.messages.len());

        // Verify the updated values in the storage
        assert_eq!(
            query_pyth_feeder_config(deps.as_ref(), "usei".to_string()).unwrap(),
            PythFeederConfigResponse {
                price_feed_id: PriceIdentifier::from_hex(
                    "ff61491a931112ddf1bd8147cd1b641375f79f5825126d665480874634fd0acd"
                )
                .unwrap(),
                price_feed_symbol: "Crypto.ETH/USD".to_string(),
                price_feed_decimal: 8,
                price_feed_age: 60,
                check_feed_age: true,
                is_valid: true,
            }
        );

        let _msg = ExecuteMsg::SetConfigFeedValid {
            asset: "usei".to_string(),
            valid: false,
        };
        let _info = mock_info("random_user", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg.clone());
        match _res {
            Err(ContractError::Unauthorized { .. }) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // Positive test case
        let _info = mock_info("creator", &[]);
        let _res = execute(deps.as_mut(), mock_env(), _info, _msg.clone()).unwrap();
        assert_eq!(0, _res.messages.len());

        // Verify the updated values in the storage
        assert_eq!(
            query_pyth_feeder_config(deps.as_ref(), "usei".to_string()).unwrap(),
            PythFeederConfigResponse {
                price_feed_id: PriceIdentifier::from_hex(
                    "ff61491a931112ddf1bd8147cd1b641375f79f5825126d665480874634fd0acd"
                )
                .unwrap(),
                price_feed_symbol: "Crypto.ETH/USD".to_string(),
                price_feed_decimal: 8,
                price_feed_age: 60,
                check_feed_age: true,
                is_valid: false,
            }
        );
    }
}
