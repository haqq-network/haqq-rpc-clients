# \MsgApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**clawback**](MsgApi.md#clawback) | **GET** /haqq/vesting/v1/tx/clawback | Clawback removes the unvested tokens from a ClawbackVestingAccount.
[**convert_coin**](MsgApi.md#convert_coin) | **GET** /evmos/erc20/v1/tx/convert_coin | ConvertCoin mints a ERC20 representation of the native Cosmos coin denom that is registered on the token mapping.
[**convert_erc20**](MsgApi.md#convert_erc20) | **GET** /evmos/erc20/v1/tx/convert_erc20 | ConvertERC20 mints a native Cosmos coin representation of the ERC20 token contract that is registered on the token mapping.
[**convert_into_vesting_account**](MsgApi.md#convert_into_vesting_account) | **GET** /haqq/vesting/v1/tx/convert_into_vesting_account | ConvertIntoVestingAccount converts a Eth account to a ClawbackVestingAccount
[**convert_vesting_account**](MsgApi.md#convert_vesting_account) | **GET** /haqq/vesting/v1/tx/convert_vesting_account | ConvertVestingAccount converts a ClawbackVestingAccount to a Eth account
[**create_clawback_vesting_account**](MsgApi.md#create_clawback_vesting_account) | **GET** /haqq/vesting/v1/tx/create_clawback_vesting_account | CreateClawbackVestingAccount creats a vesting account that is subject to clawback and the configuration of vesting and lockup schedules.
[**ethereum_tx**](MsgApi.md#ethereum_tx) | **POST** /evmos/evm/v1/ethereum_tx | EthereumTx defines a method submitting Ethereum transactions.
[**update_vesting_funder**](MsgApi.md#update_vesting_funder) | **GET** /haqq/vesting/v1/tx/update_vesting_funder | UpdateVestingFunder updates the funder address of an existing ClawbackVestingAccount.



## clawback

> serde_json::Value clawback(funder_address, account_address, dest_address)
Clawback removes the unvested tokens from a ClawbackVestingAccount.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**funder_address** | Option<**String**> | funder_address is the address which funded the account |  |
**account_address** | Option<**String**> | account_address is the address of the ClawbackVestingAccount to claw back from. |  |
**dest_address** | Option<**String**> | dest_address specifies where the clawed-back tokens should be transferred to. If empty, the tokens will be transferred back to the original funder of the account. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## convert_coin

> serde_json::Value convert_coin(coin_period_denom, coin_period_amount, receiver, sender)
ConvertCoin mints a ERC20 representation of the native Cosmos coin denom that is registered on the token mapping.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**coin_period_denom** | Option<**String**> |  |  |
**coin_period_amount** | Option<**String**> |  |  |
**receiver** | Option<**String**> | receiver is the hex address to receive ERC20 token |  |
**sender** | Option<**String**> | sender is the cosmos bech32 address from the owner of the given Cosmos coins |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## convert_erc20

> serde_json::Value convert_erc20(contract_address, amount, receiver, sender)
ConvertERC20 mints a native Cosmos coin representation of the ERC20 token contract that is registered on the token mapping.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_address** | Option<**String**> | contract_address of an ERC20 token contract, that is registered in a token pair |  |
**amount** | Option<**String**> | amount of ERC20 tokens to convert |  |
**receiver** | Option<**String**> | receiver is the bech32 address to receive native Cosmos coins |  |
**sender** | Option<**String**> | sender is the hex address from the owner of the given ERC20 tokens |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## convert_into_vesting_account

> serde_json::Value convert_into_vesting_account(from_address, to_address, start_time, merge, stake, validator_address)
ConvertIntoVestingAccount converts a Eth account to a ClawbackVestingAccount

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from_address** | Option<**String**> | from_address specifies the account to provide the funds and sign the clawback request |  |
**to_address** | Option<**String**> | to_address is the account to be converted into clawback vesting account |  |
**start_time** | Option<**String**> | start_time defines the time at which the vesting period begins |  |
**merge** | Option<**bool**> | merge specifies a the conversion mechanism for existing ClawbackVestingAccounts. If true, merge this new grant into an existing ClawbackVestingAccount, or create it if it does not exist. If false, creates a new account. New grants to an existing account must be from the same from_address. |  |
**stake** | Option<**bool**> | stake specifies a the post-creation flow. If true, delegate the total amount to a specified validator. If false, do nothing. |  |
**validator_address** | Option<**String**> | validator_address specifies the validator to delegate tokens to. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## convert_vesting_account

> serde_json::Value convert_vesting_account(vesting_address)
ConvertVestingAccount converts a ClawbackVestingAccount to a Eth account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vesting_address** | Option<**String**> | vesting_address is the address of the vesting account to convert |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_clawback_vesting_account

> serde_json::Value create_clawback_vesting_account(from_address, to_address, start_time, merge)
CreateClawbackVestingAccount creats a vesting account that is subject to clawback and the configuration of vesting and lockup schedules.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from_address** | Option<**String**> | from_address specifies the account to provide the funds and sign the clawback request |  |
**to_address** | Option<**String**> | to_address specifies the account to receive the funds |  |
**start_time** | Option<**String**> | start_time defines the time at which the vesting period begins |  |
**merge** | Option<**bool**> | merge specifies a the creation mechanism for existing ClawbackVestingAccounts. If true, merge this new grant into an existing ClawbackVestingAccount, or create it if it does not exist. If false, creates a new account. New grants to an existing account must be from the same from_address. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ethereum_tx

> crate::models::EthermintPeriodEvmPeriodV1PeriodMsgEthereumTxResponse ethereum_tx(data_period_type_url, data_period_value, size, hash, from)
EthereumTx defines a method submitting Ethereum transactions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_period_type_url** | Option<**String**> | A URL/resource name that uniquely identifies the type of the serialized protocol buffer message. This string must contain at least one \"/\" character. The last segment of the URL's path must represent the fully qualified name of the type (as in `path/google.protobuf.Duration`). The name should be in a canonical form (e.g., leading \".\" is not accepted).  In practice, teams usually precompile into the binary all types that they expect it to use in the context of Any. However, for URLs which use the scheme `http`, `https`, or no scheme, one can optionally set up a type server that maps type URLs to message definitions as follows:  * If no scheme is provided, `https` is assumed. * An HTTP GET on the URL must yield a [google.protobuf.Type][]   value in binary format, or produce an error. * Applications are allowed to cache lookup results based on the   URL, or have them precompiled into a binary to avoid any   lookup. Therefore, binary compatibility needs to be preserved   on changes to types. (Use versioned type names to manage   breaking changes.)  Note: this functionality is not currently available in the official protobuf release, and it is not used for type URLs beginning with type.googleapis.com.  Schemes other than `http`, `https` (or the empty scheme) might be used with implementation specific semantics. |  |
**data_period_value** | Option<**String**> | Must be a valid serialized protocol buffer of the above specified type. |  |
**size** | Option<**f64**> | size is the encoded storage size of the transaction (DEPRECATED) |  |
**hash** | Option<**String**> | hash of the transaction in hex format |  |
**from** | Option<**String**> | from is the ethereum signer address in hex format. This address value is checked against the address derived from the signature (V, R, S) using the secp256k1 elliptic curve |  |

### Return type

[**crate::models::EthermintPeriodEvmPeriodV1PeriodMsgEthereumTxResponse**](ethermint.evm.v1.MsgEthereumTxResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_vesting_funder

> serde_json::Value update_vesting_funder(funder_address, new_funder_address, vesting_address)
UpdateVestingFunder updates the funder address of an existing ClawbackVestingAccount.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**funder_address** | Option<**String**> | funder_address is the current funder address of the ClawbackVestingAccount |  |
**new_funder_address** | Option<**String**> | new_funder_address is the new address to replace the existing funder_address |  |
**vesting_address** | Option<**String**> | vesting_address is the address of the ClawbackVestingAccount being updated |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

