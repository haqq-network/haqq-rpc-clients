# \ServiceApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**a_bci_query**](ServiceApi.md#a_bci_query) | **GET** /cosmos/base/tendermint/v1beta1/abci_query | ABCIQuery defines a query handler that supports ABCI queries directly to the application, bypassing Tendermint completely. The ABCI query must contain a valid and supported path, including app, custom, p2p, and store.
[**broadcast_tx**](ServiceApi.md#broadcast_tx) | **POST** /cosmos/tx/v1beta1/txs | BroadcastTx broadcast transaction.
[**config**](ServiceApi.md#config) | **GET** /cosmos/base/node/v1beta1/config | Config queries for the operator configuration.
[**get_block_by_height**](ServiceApi.md#get_block_by_height) | **GET** /cosmos/base/tendermint/v1beta1/blocks/{height} | GetBlockByHeight queries block for given height.
[**get_block_with_txs**](ServiceApi.md#get_block_with_txs) | **GET** /cosmos/tx/v1beta1/txs/block/{height} | GetBlockWithTxs fetches a block with decoded txs.
[**get_latest_block**](ServiceApi.md#get_latest_block) | **GET** /cosmos/base/tendermint/v1beta1/blocks/latest | GetLatestBlock returns the latest block.
[**get_latest_validator_set**](ServiceApi.md#get_latest_validator_set) | **GET** /cosmos/base/tendermint/v1beta1/validatorsets/latest | GetLatestValidatorSet queries latest validator-set.
[**get_node_info**](ServiceApi.md#get_node_info) | **GET** /cosmos/base/tendermint/v1beta1/node_info | GetNodeInfo queries the current node info.
[**get_syncing**](ServiceApi.md#get_syncing) | **GET** /cosmos/base/tendermint/v1beta1/syncing | GetSyncing queries node syncing.
[**get_tx**](ServiceApi.md#get_tx) | **GET** /cosmos/tx/v1beta1/txs/{hash} | GetTx fetches a tx by hash.
[**get_txs_event**](ServiceApi.md#get_txs_event) | **GET** /cosmos/tx/v1beta1/txs | GetTxsEvent fetches txs by event.
[**get_validator_set_by_height**](ServiceApi.md#get_validator_set_by_height) | **GET** /cosmos/base/tendermint/v1beta1/validatorsets/{height} | GetValidatorSetByHeight queries validator-set at a given height.
[**simulate**](ServiceApi.md#simulate) | **POST** /cosmos/tx/v1beta1/simulate | Simulate simulates executing a transaction for estimating gas usage.



## a_bci_query

> crate::models::CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodAbciQueryResponse a_bci_query(data, path, height, prove)
ABCIQuery defines a query handler that supports ABCI queries directly to the application, bypassing Tendermint completely. The ABCI query must contain a valid and supported path, including app, custom, p2p, and store.

Since: cosmos-sdk 0.46

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | Option<**String**> |  |  |
**path** | Option<**String**> |  |  |
**height** | Option<**String**> |  |  |
**prove** | Option<**bool**> |  |  |

### Return type

[**crate::models::CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodAbciQueryResponse**](cosmos.base.tendermint.v1beta1.ABCIQueryResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## broadcast_tx

> crate::models::CosmosPeriodTxPeriodV1beta1PeriodBroadcastTxResponse broadcast_tx(body)
BroadcastTx broadcast transaction.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CosmosPeriodTxPeriodV1beta1PeriodBroadcastTxRequest**](CosmosPeriodTxPeriodV1beta1PeriodBroadcastTxRequest.md) | BroadcastTxRequest is the request type for the Service.BroadcastTxRequest RPC method. | [required] |

### Return type

[**crate::models::CosmosPeriodTxPeriodV1beta1PeriodBroadcastTxResponse**](cosmos.tx.v1beta1.BroadcastTxResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## config

> crate::models::CosmosPeriodBasePeriodNodePeriodV1beta1PeriodConfigResponse config()
Config queries for the operator configuration.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CosmosPeriodBasePeriodNodePeriodV1beta1PeriodConfigResponse**](cosmos.base.node.v1beta1.ConfigResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_block_by_height

> crate::models::CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodGetBlockByHeightResponse get_block_by_height(height)
GetBlockByHeight queries block for given height.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**height** | **String** |  | [required] |

### Return type

[**crate::models::CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodGetBlockByHeightResponse**](cosmos.base.tendermint.v1beta1.GetBlockByHeightResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_block_with_txs

> crate::models::CosmosPeriodTxPeriodV1beta1PeriodGetBlockWithTxsResponse get_block_with_txs(height, pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
GetBlockWithTxs fetches a block with decoded txs.

Since: cosmos-sdk 0.45.2

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**height** | **String** | height is the height of the block to query. | [required] |
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodTxPeriodV1beta1PeriodGetBlockWithTxsResponse**](cosmos.tx.v1beta1.GetBlockWithTxsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_latest_block

> crate::models::CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodGetLatestBlockResponse get_latest_block()
GetLatestBlock returns the latest block.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodGetLatestBlockResponse**](cosmos.base.tendermint.v1beta1.GetLatestBlockResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_latest_validator_set

> crate::models::CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodGetLatestValidatorSetResponse get_latest_validator_set(pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
GetLatestValidatorSet queries latest validator-set.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodGetLatestValidatorSetResponse**](cosmos.base.tendermint.v1beta1.GetLatestValidatorSetResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_node_info

> crate::models::CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodGetNodeInfoResponse get_node_info()
GetNodeInfo queries the current node info.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodGetNodeInfoResponse**](cosmos.base.tendermint.v1beta1.GetNodeInfoResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_syncing

> crate::models::CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodGetSyncingResponse get_syncing()
GetSyncing queries node syncing.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodGetSyncingResponse**](cosmos.base.tendermint.v1beta1.GetSyncingResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tx

> crate::models::CosmosPeriodTxPeriodV1beta1PeriodGetTxResponse get_tx(hash)
GetTx fetches a tx by hash.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hash** | **String** | hash is the tx hash to query, encoded as a hex string. | [required] |

### Return type

[**crate::models::CosmosPeriodTxPeriodV1beta1PeriodGetTxResponse**](cosmos.tx.v1beta1.GetTxResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_txs_event

> crate::models::CosmosPeriodTxPeriodV1beta1PeriodGetTxsEventResponse get_txs_event(events, pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse, order_by, page, limit)
GetTxsEvent fetches txs by event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**events** | Option<[**Vec<String>**](String.md)> | events is the list of transaction event type. |  |
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |
**order_by** | Option<**String**> |  - ORDER_BY_UNSPECIFIED: ORDER_BY_UNSPECIFIED specifies an unknown sorting order. OrderBy defaults to ASC in this case.  - ORDER_BY_ASC: ORDER_BY_ASC defines ascending order  - ORDER_BY_DESC: ORDER_BY_DESC defines descending order |  |[default to ORDER_BY_UNSPECIFIED]
**page** | Option<**String**> | page is the page number to query, starts at 1. If not provided, will default to first page. |  |
**limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |

### Return type

[**crate::models::CosmosPeriodTxPeriodV1beta1PeriodGetTxsEventResponse**](cosmos.tx.v1beta1.GetTxsEventResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_validator_set_by_height

> crate::models::CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodGetValidatorSetByHeightResponse get_validator_set_by_height(height, pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
GetValidatorSetByHeight queries validator-set at a given height.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**height** | **String** |  | [required] |
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodBasePeriodTendermintPeriodV1beta1PeriodGetValidatorSetByHeightResponse**](cosmos.base.tendermint.v1beta1.GetValidatorSetByHeightResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## simulate

> crate::models::CosmosPeriodTxPeriodV1beta1PeriodSimulateResponse simulate(body)
Simulate simulates executing a transaction for estimating gas usage.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CosmosPeriodTxPeriodV1beta1PeriodSimulateRequest**](CosmosPeriodTxPeriodV1beta1PeriodSimulateRequest.md) | SimulateRequest is the request type for the Service.Simulate RPC method. | [required] |

### Return type

[**crate::models::CosmosPeriodTxPeriodV1beta1PeriodSimulateResponse**](cosmos.tx.v1beta1.SimulateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

