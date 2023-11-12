/*
 * cosmos/app/v1alpha1/module.proto
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`a_bci_query`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ABciQueryError {
    DefaultResponse(crate::models::GooglePeriodRpcPeriodStatus),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`broadcast_tx`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BroadcastTxError {
    DefaultResponse(crate::models::GooglePeriodRpcPeriodStatus),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConfigError {
    DefaultResponse(crate::models::GooglePeriodRpcPeriodStatus),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_block_by_height`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBlockByHeightError {
    DefaultResponse(crate::models::GooglePeriodRpcPeriodStatus),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_block_with_txs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBlockWithTxsError {
    DefaultResponse(crate::models::GooglePeriodRpcPeriodStatus),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_latest_block`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLatestBlockError {
    DefaultResponse(crate::models::GooglePeriodRpcPeriodStatus),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_latest_validator_set`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLatestValidatorSetError {
    DefaultResponse(crate::models::GooglePeriodRpcPeriodStatus),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_node_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetNodeInfoError {
    DefaultResponse(crate::models::GooglePeriodRpcPeriodStatus),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_syncing`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSyncingError {
    DefaultResponse(crate::models::GooglePeriodRpcPeriodStatus),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_tx`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTxError {
    DefaultResponse(crate::models::GooglePeriodRpcPeriodStatus),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_txs_event`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTxsEventError {
    DefaultResponse(crate::models::GooglePeriodRpcPeriodStatus),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_validator_set_by_height`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetValidatorSetByHeightError {
    DefaultResponse(crate::models::GooglePeriodRpcPeriodStatus),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`simulate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SimulateError {
    DefaultResponse(crate::models::GooglePeriodRpcPeriodStatus),
    UnknownValue(serde_json::Value),
}


/// Since: cosmos-sdk 0.46
pub async fn a_bci_query(configuration: &configuration::Configuration, data: Option<String>, path: Option<&str>, height: Option<&str>, prove: Option<bool>) -> Result<crate::models::CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodAbciQueryResponse, Error<ABciQueryError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cosmos/base/tendermint/v1beta1/abci_query", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = data {
        local_var_req_builder = local_var_req_builder.query(&[("data", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = path {
        local_var_req_builder = local_var_req_builder.query(&[("path", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = height {
        local_var_req_builder = local_var_req_builder.query(&[("height", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = prove {
        local_var_req_builder = local_var_req_builder.query(&[("prove", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ABciQueryError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn broadcast_tx(configuration: &configuration::Configuration, body: crate::models::CosmosPeriodTxPeriodV1beta1PeriodBroadcastTxRequest) -> Result<crate::models::CosmosPeriodTxPeriodV1beta1PeriodBroadcastTxResponse, Error<BroadcastTxError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cosmos/tx/v1beta1/txs", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<BroadcastTxError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn config(configuration: &configuration::Configuration, ) -> Result<crate::models::CosmosPeriodBasePeriodNodePeriodV1beta1PeriodConfigResponse, Error<ConfigError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cosmos/base/node/v1beta1/config", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ConfigError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_block_by_height(configuration: &configuration::Configuration, height: &str) -> Result<crate::models::CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodGetBlockByHeightResponse, Error<GetBlockByHeightError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cosmos/base/tendermint/v1beta1/blocks/{height}", local_var_configuration.base_path, height=crate::apis::urlencode(height));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetBlockByHeightError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Since: cosmos-sdk 0.45.2
pub async fn get_block_with_txs(configuration: &configuration::Configuration, height: &str, pagination_period_key: Option<String>, pagination_period_offset: Option<&str>, pagination_period_limit: Option<&str>, pagination_period_count_total: Option<bool>, pagination_period_reverse: Option<bool>) -> Result<crate::models::CosmosPeriodTxPeriodV1beta1PeriodGetBlockWithTxsResponse, Error<GetBlockWithTxsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cosmos/tx/v1beta1/txs/block/{height}", local_var_configuration.base_path, height=crate::apis::urlencode(height));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = pagination_period_key {
        local_var_req_builder = local_var_req_builder.query(&[("pagination.key", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pagination_period_offset {
        local_var_req_builder = local_var_req_builder.query(&[("pagination.offset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pagination_period_limit {
        local_var_req_builder = local_var_req_builder.query(&[("pagination.limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pagination_period_count_total {
        local_var_req_builder = local_var_req_builder.query(&[("pagination.countTotal", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pagination_period_reverse {
        local_var_req_builder = local_var_req_builder.query(&[("pagination.reverse", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetBlockWithTxsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_latest_block(configuration: &configuration::Configuration, ) -> Result<crate::models::CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodGetLatestBlockResponse, Error<GetLatestBlockError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cosmos/base/tendermint/v1beta1/blocks/latest", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetLatestBlockError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_latest_validator_set(configuration: &configuration::Configuration, pagination_period_key: Option<String>, pagination_period_offset: Option<&str>, pagination_period_limit: Option<&str>, pagination_period_count_total: Option<bool>, pagination_period_reverse: Option<bool>) -> Result<crate::models::CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodGetLatestValidatorSetResponse, Error<GetLatestValidatorSetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cosmos/base/tendermint/v1beta1/validatorsets/latest", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = pagination_period_key {
        local_var_req_builder = local_var_req_builder.query(&[("pagination.key", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pagination_period_offset {
        local_var_req_builder = local_var_req_builder.query(&[("pagination.offset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pagination_period_limit {
        local_var_req_builder = local_var_req_builder.query(&[("pagination.limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pagination_period_count_total {
        local_var_req_builder = local_var_req_builder.query(&[("pagination.countTotal", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pagination_period_reverse {
        local_var_req_builder = local_var_req_builder.query(&[("pagination.reverse", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetLatestValidatorSetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_node_info(configuration: &configuration::Configuration, ) -> Result<crate::models::CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodGetNodeInfoResponse, Error<GetNodeInfoError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cosmos/base/tendermint/v1beta1/node_info", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetNodeInfoError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_syncing(configuration: &configuration::Configuration, ) -> Result<crate::models::CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodGetSyncingResponse, Error<GetSyncingError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cosmos/base/tendermint/v1beta1/syncing", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetSyncingError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_tx(configuration: &configuration::Configuration, hash: &str) -> Result<crate::models::CosmosPeriodTxPeriodV1beta1PeriodGetTxResponse, Error<GetTxError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cosmos/tx/v1beta1/txs/{hash}", local_var_configuration.base_path, hash=crate::apis::urlencode(hash));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetTxError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_txs_event(configuration: &configuration::Configuration, events: Option<Vec<String>>, pagination_period_key: Option<String>, pagination_period_offset: Option<&str>, pagination_period_limit: Option<&str>, pagination_period_count_total: Option<bool>, pagination_period_reverse: Option<bool>, order_by: Option<&str>, page: Option<&str>, limit: Option<&str>) -> Result<crate::models::CosmosPeriodTxPeriodV1beta1PeriodGetTxsEventResponse, Error<GetTxsEventError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cosmos/tx/v1beta1/txs", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = events {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("events".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("events", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = pagination_period_key {
        local_var_req_builder = local_var_req_builder.query(&[("pagination.key", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pagination_period_offset {
        local_var_req_builder = local_var_req_builder.query(&[("pagination.offset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pagination_period_limit {
        local_var_req_builder = local_var_req_builder.query(&[("pagination.limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pagination_period_count_total {
        local_var_req_builder = local_var_req_builder.query(&[("pagination.countTotal", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pagination_period_reverse {
        local_var_req_builder = local_var_req_builder.query(&[("pagination.reverse", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = order_by {
        local_var_req_builder = local_var_req_builder.query(&[("orderBy", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetTxsEventError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_validator_set_by_height(configuration: &configuration::Configuration, height: &str, pagination_period_key: Option<String>, pagination_period_offset: Option<&str>, pagination_period_limit: Option<&str>, pagination_period_count_total: Option<bool>, pagination_period_reverse: Option<bool>) -> Result<crate::models::CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodGetValidatorSetByHeightResponse, Error<GetValidatorSetByHeightError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cosmos/base/tendermint/v1beta1/validatorsets/{height}", local_var_configuration.base_path, height=crate::apis::urlencode(height));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = pagination_period_key {
        local_var_req_builder = local_var_req_builder.query(&[("pagination.key", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pagination_period_offset {
        local_var_req_builder = local_var_req_builder.query(&[("pagination.offset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pagination_period_limit {
        local_var_req_builder = local_var_req_builder.query(&[("pagination.limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pagination_period_count_total {
        local_var_req_builder = local_var_req_builder.query(&[("pagination.countTotal", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pagination_period_reverse {
        local_var_req_builder = local_var_req_builder.query(&[("pagination.reverse", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetValidatorSetByHeightError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn simulate(configuration: &configuration::Configuration, body: crate::models::CosmosPeriodTxPeriodV1beta1PeriodSimulateRequest) -> Result<crate::models::CosmosPeriodTxPeriodV1beta1PeriodSimulateResponse, Error<SimulateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cosmos/tx/v1beta1/simulate", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SimulateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
