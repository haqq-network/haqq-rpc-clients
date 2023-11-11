# \QueryApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**account**](QueryApi.md#account) | **GET** /cosmos/auth/v1beta1/accounts/{address} | Account returns account details based on address.
[**account_0**](QueryApi.md#account_0) | **GET** /evmos/evm/v1/account/{address} | Account queries an Ethereum account.
[**account_address_by_id**](QueryApi.md#account_address_by_id) | **GET** /cosmos/auth/v1beta1/address_by_id/{id} | AccountAddressByID returns account address based on account number.
[**accounts**](QueryApi.md#accounts) | **GET** /cosmos/auth/v1beta1/accounts | Accounts returns all the existing accounts
[**address_bytes_to_string**](QueryApi.md#address_bytes_to_string) | **GET** /cosmos/auth/v1beta1/bech32/{addressBytes} | AddressBytesToString converts Account Address bytes to string
[**address_string_to_bytes**](QueryApi.md#address_string_to_bytes) | **GET** /cosmos/auth/v1beta1/bech32/{addressString} | AddressStringToBytes converts Address string to bytes
[**all_balances**](QueryApi.md#all_balances) | **GET** /cosmos/bank/v1beta1/balances/{address} | AllBalances queries the balance of all coins for a single account.
[**all_evidence**](QueryApi.md#all_evidence) | **GET** /cosmos/evidence/v1beta1/evidence | AllEvidence queries all evidence.
[**allowance**](QueryApi.md#allowance) | **GET** /cosmos/feegrant/v1beta1/allowance/{granter}/{grantee} | Allowance returns fee granted to the grantee by the granter.
[**allowances**](QueryApi.md#allowances) | **GET** /cosmos/feegrant/v1beta1/allowances/{grantee} | Allowances returns all the grants for address.
[**allowances_by_granter**](QueryApi.md#allowances_by_granter) | **GET** /cosmos/feegrant/v1beta1/issued/{granter} | AllowancesByGranter returns all the grants given by an address
[**annual_provisions**](QueryApi.md#annual_provisions) | **GET** /cosmos/mint/v1beta1/annual_provisions | AnnualProvisions current minting annual provisions value.
[**applied_plan**](QueryApi.md#applied_plan) | **GET** /cosmos/upgrade/v1beta1/applied_plan/{name} | AppliedPlan queries a previously applied upgrade plan by its name.
[**authority**](QueryApi.md#authority) | **GET** /cosmos/upgrade/v1beta1/authority | Returns the account with authority to conduct upgrades
[**balance**](QueryApi.md#balance) | **GET** /cosmos/bank/v1beta1/balances/{address}/by_denom | Balance queries the balance of a single coin for a single account.
[**balance_0**](QueryApi.md#balance_0) | **GET** /cosmos/nft/v1beta1/balance/{owner}/{classId} | Balance queries the number of NFTs of a given class owned by the owner, same as balanceOf in ERC721
[**balance_1**](QueryApi.md#balance_1) | **GET** /evmos/evm/v1/balances/{address} | Balance queries the balance of a the EVM denomination for a single EthAccount.
[**balances**](QueryApi.md#balances) | **GET** /haqq/vesting/v1/balances/{address} | Balances retrieves the unvested, vested and locked tokens for a vesting account
[**base_fee**](QueryApi.md#base_fee) | **GET** /evmos/evm/v1/base_fee | BaseFee queries the base fee of the parent block of the current block, it's similar to feemarket module's method, but also checks london hardfork status.
[**base_fee_0**](QueryApi.md#base_fee_0) | **GET** /evmos/feemarket/v1/base_fee | BaseFee queries the base fee of the parent block of the current block.
[**bech32_prefix**](QueryApi.md#bech32_prefix) | **GET** /cosmos/auth/v1beta1/bech32 | Bech32Prefix queries bech32Prefix
[**block_gas**](QueryApi.md#block_gas) | **GET** /evmos/feemarket/v1/block_gas | BlockGas queries the gas used at a given block height
[**class**](QueryApi.md#class) | **GET** /cosmos/nft/v1beta1/classes/{classId} | Class queries an NFT class based on its id
[**classes**](QueryApi.md#classes) | **GET** /cosmos/nft/v1beta1/classes | Classes queries all NFT classes
[**code**](QueryApi.md#code) | **GET** /evmos/evm/v1/codes/{address} | Code queries the balance of all coins for a single account.
[**community_pool**](QueryApi.md#community_pool) | **GET** /cosmos/distribution/v1beta1/community_pool | CommunityPool queries the community pool coins.
[**cosmos_account**](QueryApi.md#cosmos_account) | **GET** /evmos/evm/v1/cosmos_account/{address} | CosmosAccount queries an Ethereum account's Cosmos Address.
[**current_epoch**](QueryApi.md#current_epoch) | **GET** /evmos/epochs/v1/current_epoch | CurrentEpoch provide current epoch of specified identifier
[**current_plan**](QueryApi.md#current_plan) | **GET** /cosmos/upgrade/v1beta1/current_plan | CurrentPlan queries the current upgrade plan.
[**delegation**](QueryApi.md#delegation) | **GET** /cosmos/staking/v1beta1/validators/{validatorAddr}/delegations/{delegatorAddr} | Delegation queries delegate info for given validator delegator pair.
[**delegation_rewards**](QueryApi.md#delegation_rewards) | **GET** /cosmos/distribution/v1beta1/delegators/{delegatorAddress}/rewards/{validatorAddress} | DelegationRewards queries the total rewards accrued by a delegation.
[**delegation_total_rewards**](QueryApi.md#delegation_total_rewards) | **GET** /cosmos/distribution/v1beta1/delegators/{delegatorAddress}/rewards | DelegationTotalRewards queries the total rewards accrued by a each validator.
[**delegator_delegations**](QueryApi.md#delegator_delegations) | **GET** /cosmos/staking/v1beta1/delegations/{delegatorAddr} | DelegatorDelegations queries all delegations of a given delegator address.
[**delegator_unbonding_delegations**](QueryApi.md#delegator_unbonding_delegations) | **GET** /cosmos/staking/v1beta1/delegators/{delegatorAddr}/unbonding_delegations | DelegatorUnbondingDelegations queries all unbonding delegations of a given delegator address.
[**delegator_validator**](QueryApi.md#delegator_validator) | **GET** /cosmos/staking/v1beta1/delegators/{delegatorAddr}/validators/{validatorAddr} | DelegatorValidator queries validator info for given delegator validator pair.
[**delegator_validators**](QueryApi.md#delegator_validators) | **GET** /cosmos/distribution/v1beta1/delegators/{delegatorAddress}/validators | DelegatorValidators queries the validators of a delegator.
[**delegator_validators_0**](QueryApi.md#delegator_validators_0) | **GET** /cosmos/staking/v1beta1/delegators/{delegatorAddr}/validators | DelegatorValidators queries all validators info for given delegator address.
[**delegator_withdraw_address**](QueryApi.md#delegator_withdraw_address) | **GET** /cosmos/distribution/v1beta1/delegators/{delegatorAddress}/withdraw_address | DelegatorWithdrawAddress queries withdraw address of a delegator.
[**denom_metadata**](QueryApi.md#denom_metadata) | **GET** /cosmos/bank/v1beta1/denoms_metadata/{denom} | DenomsMetadata queries the client metadata of a given coin denomination.
[**denom_owners**](QueryApi.md#denom_owners) | **GET** /cosmos/bank/v1beta1/denom_owners/{denom} | DenomOwners queries for all account addresses that own a particular token denomination.
[**denoms_metadata**](QueryApi.md#denoms_metadata) | **GET** /cosmos/bank/v1beta1/denoms_metadata | DenomsMetadata queries the client metadata for all registered coin denominations.
[**deposit**](QueryApi.md#deposit) | **GET** /cosmos/gov/v1/proposals/{proposalId}/deposits/{depositor} | Deposit queries single deposit information based proposalID, depositAddr.
[**deposit_0**](QueryApi.md#deposit_0) | **GET** /cosmos/gov/v1beta1/proposals/{proposalId}/deposits/{depositor} | Deposit queries single deposit information based proposalID, depositAddr.
[**deposits**](QueryApi.md#deposits) | **GET** /cosmos/gov/v1/proposals/{proposalId}/deposits | Deposits queries all deposits of a single proposal.
[**deposits_0**](QueryApi.md#deposits_0) | **GET** /cosmos/gov/v1beta1/proposals/{proposalId}/deposits | Deposits queries all deposits of a single proposal.
[**epoch_infos**](QueryApi.md#epoch_infos) | **GET** /evmos/epochs/v1/epochs | EpochInfos provide running epochInfos
[**era**](QueryApi.md#era) | **GET** /haqqd/coinomics/v1/era | Era retrieves current era.
[**era_closing_supply**](QueryApi.md#era_closing_supply) | **GET** /haqqd/coinomics/v1/era_closing_supply | EraTargetSupply retrieves current era target supply.
[**estimate_gas**](QueryApi.md#estimate_gas) | **GET** /evmos/evm/v1/estimate_gas | EstimateGas implements the `eth_estimateGas` rpc api
[**eth_call**](QueryApi.md#eth_call) | **GET** /evmos/evm/v1/eth_call | EthCall implements the `eth_call` rpc api
[**evidence**](QueryApi.md#evidence) | **GET** /cosmos/evidence/v1beta1/evidence/{evidenceHash} | Evidence queries evidence based on evidence hash.
[**grantee_grants**](QueryApi.md#grantee_grants) | **GET** /cosmos/authz/v1beta1/grants/grantee/{grantee} | GranteeGrants returns a list of `GrantAuthorization` by grantee.
[**granter_grants**](QueryApi.md#granter_grants) | **GET** /cosmos/authz/v1beta1/grants/granter/{granter} | GranterGrants returns list of `GrantAuthorization`, granted by granter.
[**grants**](QueryApi.md#grants) | **GET** /cosmos/authz/v1beta1/grants | Returns list of `Authorization`, granted to the grantee by the granter.
[**group_info**](QueryApi.md#group_info) | **GET** /cosmos/group/v1/group_info/{groupId} | GroupInfo queries group info based on group id.
[**group_members**](QueryApi.md#group_members) | **GET** /cosmos/group/v1/group_members/{groupId} | GroupMembers queries members of a group
[**group_policies_by_admin**](QueryApi.md#group_policies_by_admin) | **GET** /cosmos/group/v1/group_policies_by_admin/{admin} | GroupsByAdmin queries group policies by admin address.
[**group_policies_by_group**](QueryApi.md#group_policies_by_group) | **GET** /cosmos/group/v1/group_policies_by_group/{groupId} | GroupPoliciesByGroup queries group policies by group id.
[**group_policy_info**](QueryApi.md#group_policy_info) | **GET** /cosmos/group/v1/group_policy_info/{address} | GroupPolicyInfo queries group policy info based on account address of group policy.
[**groups**](QueryApi.md#groups) | **GET** /cosmos/group/v1/groups | Groups queries all groups in state.
[**groups_by_admin**](QueryApi.md#groups_by_admin) | **GET** /cosmos/group/v1/groups_by_admin/{admin} | GroupsByAdmin queries groups by admin address.
[**groups_by_member**](QueryApi.md#groups_by_member) | **GET** /cosmos/group/v1/groups_by_member/{address} | GroupsByMember queries groups by member address.
[**historical_info**](QueryApi.md#historical_info) | **GET** /cosmos/staking/v1beta1/historical_info/{height} | HistoricalInfo queries the historical info for given height.
[**inflation**](QueryApi.md#inflation) | **GET** /cosmos/mint/v1beta1/inflation | Inflation returns the current minting inflation value.
[**inflation_rate**](QueryApi.md#inflation_rate) | **GET** /haqq/coinomics/v1/inflation_rate | InflationRate retrieves current era inflation rate.
[**max_supply**](QueryApi.md#max_supply) | **GET** /haqqd/coinomics/v1/max_supply | MaxSupply retrieves total coins of all eras and when mint ended.
[**module_account_by_name**](QueryApi.md#module_account_by_name) | **GET** /cosmos/auth/v1beta1/module_accounts/{name} | ModuleAccountByName returns the module account info by module name
[**module_accounts**](QueryApi.md#module_accounts) | **GET** /cosmos/auth/v1beta1/module_accounts | ModuleAccounts returns all the existing module accounts.
[**module_versions**](QueryApi.md#module_versions) | **GET** /cosmos/upgrade/v1beta1/module_versions | ModuleVersions queries the list of module versions from state.
[**n_ft**](QueryApi.md#n_ft) | **GET** /cosmos/nft/v1beta1/nfts/{classId}/{id} | NFT queries an NFT based on its class and id.
[**n_fts**](QueryApi.md#n_fts) | **GET** /cosmos/nft/v1beta1/nfts | NFTs queries all NFTs of a given class or owner,choose at least one of the two, similar to tokenByIndex in ERC721Enumerable
[**owner**](QueryApi.md#owner) | **GET** /cosmos/nft/v1beta1/owner/{classId}/{id} | Owner queries the owner of the NFT based on its class and id, same as ownerOf in ERC721
[**params**](QueryApi.md#params) | **GET** /cosmos/auth/v1beta1/params | Params queries all parameters.
[**params_0**](QueryApi.md#params_0) | **GET** /cosmos/bank/v1beta1/params | Params queries the parameters of x/bank module.
[**params_1**](QueryApi.md#params_1) | **GET** /cosmos/distribution/v1beta1/params | Params queries params of the distribution module.
[**params_10**](QueryApi.md#params_10) | **GET** /evmos/feemarket/v1/params | Params queries the parameters of x/feemarket module.
[**params_11**](QueryApi.md#params_11) | **GET** /haqq/coinomics/v1/params | Params retrieves coinomics moudle params.
[**params_2**](QueryApi.md#params_2) | **GET** /cosmos/gov/v1/params/{paramsType} | Params queries all parameters of the gov module.
[**params_3**](QueryApi.md#params_3) | **GET** /cosmos/gov/v1beta1/params/{paramsType} | Params queries all parameters of the gov module.
[**params_4**](QueryApi.md#params_4) | **GET** /cosmos/mint/v1beta1/params | Params returns the total set of minting parameters.
[**params_5**](QueryApi.md#params_5) | **GET** /cosmos/params/v1beta1/params | Params queries a specific parameter of a module, given its subspace and key.
[**params_6**](QueryApi.md#params_6) | **GET** /cosmos/slashing/v1beta1/params | Params queries the parameters of slashing module
[**params_7**](QueryApi.md#params_7) | **GET** /cosmos/staking/v1beta1/params | Parameters queries the staking parameters.
[**params_8**](QueryApi.md#params_8) | **GET** /evmos/erc20/v1/params | Params retrieves the erc20 module params
[**params_9**](QueryApi.md#params_9) | **GET** /evmos/evm/v1/params | Params queries the parameters of x/evm module.
[**pool**](QueryApi.md#pool) | **GET** /cosmos/staking/v1beta1/pool | Pool queries the pool info.
[**proposal**](QueryApi.md#proposal) | **GET** /cosmos/gov/v1/proposals/{proposalId} | Proposal queries proposal details based on ProposalID.
[**proposal_0**](QueryApi.md#proposal_0) | **GET** /cosmos/gov/v1beta1/proposals/{proposalId} | Proposal queries proposal details based on ProposalID.
[**proposal_1**](QueryApi.md#proposal_1) | **GET** /cosmos/group/v1/proposal/{proposalId} | Proposal queries a proposal based on proposal id.
[**proposals**](QueryApi.md#proposals) | **GET** /cosmos/gov/v1/proposals | Proposals queries all proposals based on given status.
[**proposals_0**](QueryApi.md#proposals_0) | **GET** /cosmos/gov/v1beta1/proposals | Proposals queries all proposals based on given status.
[**proposals_by_group_policy**](QueryApi.md#proposals_by_group_policy) | **GET** /cosmos/group/v1/proposals_by_group_policy/{address} | ProposalsByGroupPolicy queries proposals based on account address of group policy.
[**redelegations**](QueryApi.md#redelegations) | **GET** /cosmos/staking/v1beta1/delegators/{delegatorAddr}/redelegations | Redelegations queries redelegations of given address.
[**signing_info**](QueryApi.md#signing_info) | **GET** /cosmos/slashing/v1beta1/signing_infos/{consAddress} | SigningInfo queries the signing info of given cons address
[**signing_infos**](QueryApi.md#signing_infos) | **GET** /cosmos/slashing/v1beta1/signing_infos | SigningInfos queries signing info of all validators
[**spendable_balances**](QueryApi.md#spendable_balances) | **GET** /cosmos/bank/v1beta1/spendable_balances/{address} | SpendableBalances queries the spenable balance of all coins for a single account.
[**storage**](QueryApi.md#storage) | **GET** /evmos/evm/v1/storage/{address}/{key} | Storage queries the balance of all coins for a single account.
[**subspaces**](QueryApi.md#subspaces) | **GET** /cosmos/params/v1beta1/subspaces | Subspaces queries for all registered subspaces and all keys for a subspace.
[**supply**](QueryApi.md#supply) | **GET** /cosmos/nft/v1beta1/supply/{classId} | Supply queries the number of NFTs from the given class, same as totalSupply of ERC721.
[**supply_of**](QueryApi.md#supply_of) | **GET** /cosmos/bank/v1beta1/supply/by_denom | SupplyOf queries the supply of a single coin.
[**tally_result**](QueryApi.md#tally_result) | **GET** /cosmos/gov/v1/proposals/{proposalId}/tally | TallyResult queries the tally of a proposal vote.
[**tally_result_0**](QueryApi.md#tally_result_0) | **GET** /cosmos/gov/v1beta1/proposals/{proposalId}/tally | TallyResult queries the tally of a proposal vote.
[**tally_result_1**](QueryApi.md#tally_result_1) | **GET** /cosmos/group/v1/proposals/{proposalId}/tally | TallyResult returns the tally result of a proposal. If the proposal is still in voting period, then this query computes the current tally state, which might not be final. On the other hand, if the proposal is final, then it simply returns the `final_tally_result` state stored in the proposal itself.
[**token_pair**](QueryApi.md#token_pair) | **GET** /evmos/erc20/v1/token_pairs/{token} | TokenPair retrieves a registered token pair
[**token_pairs**](QueryApi.md#token_pairs) | **GET** /evmos/erc20/v1/token_pairs | TokenPairs retrieves registered token pairs
[**total_locked**](QueryApi.md#total_locked) | **GET** /haqq/vesting/v1/total_locked | TotalLocked retrieves the total summary of all unvested, vested and locked tokens
[**total_supply**](QueryApi.md#total_supply) | **GET** /cosmos/bank/v1beta1/supply | TotalSupply queries the total supply of all coins.
[**trace_block**](QueryApi.md#trace_block) | **GET** /evmos/evm/v1/trace_block | TraceBlock implements the `debug_traceBlockByNumber` and `debug_traceBlockByHash` rpc api
[**trace_tx**](QueryApi.md#trace_tx) | **GET** /evmos/evm/v1/trace_tx | TraceTx implements the `debug_traceTransaction` rpc api
[**unbonding_delegation**](QueryApi.md#unbonding_delegation) | **GET** /cosmos/staking/v1beta1/validators/{validatorAddr}/delegations/{delegatorAddr}/unbonding_delegation | UnbondingDelegation queries unbonding info for given validator delegator pair.
[**upgraded_consensus_state**](QueryApi.md#upgraded_consensus_state) | **GET** /cosmos/upgrade/v1beta1/upgraded_consensus_state/{lastHeight} | UpgradedConsensusState queries the consensus state that will serve as a trusted kernel for the next version of this chain. It will only be stored at the last height of this chain. UpgradedConsensusState RPC not supported with legacy querier This rpc is deprecated now that IBC has its own replacement (https://github.com/cosmos/ibc-go/blob/2c880a22e9f9cc75f62b527ca94aa75ce1106001/proto/ibc/core/client/v1/query.proto#L54)
[**validator**](QueryApi.md#validator) | **GET** /cosmos/staking/v1beta1/validators/{validatorAddr} | Validator queries validator info for given validator address.
[**validator_account**](QueryApi.md#validator_account) | **GET** /evmos/evm/v1/validator_account/{consAddress} | ValidatorAccount queries an Ethereum account's from a validator consensus Address.
[**validator_commission**](QueryApi.md#validator_commission) | **GET** /cosmos/distribution/v1beta1/validators/{validatorAddress}/commission | ValidatorCommission queries accumulated commission for a validator.
[**validator_delegations**](QueryApi.md#validator_delegations) | **GET** /cosmos/staking/v1beta1/validators/{validatorAddr}/delegations | ValidatorDelegations queries delegate info for given validator.
[**validator_distribution_info**](QueryApi.md#validator_distribution_info) | **GET** /cosmos/distribution/v1beta1/validators/{validatorAddress} | ValidatorDistributionInfo queries validator commision and self-delegation rewards for validator
[**validator_outstanding_rewards**](QueryApi.md#validator_outstanding_rewards) | **GET** /cosmos/distribution/v1beta1/validators/{validatorAddress}/outstanding_rewards | ValidatorOutstandingRewards queries rewards of a validator address.
[**validator_slashes**](QueryApi.md#validator_slashes) | **GET** /cosmos/distribution/v1beta1/validators/{validatorAddress}/slashes | ValidatorSlashes queries slash events of a validator.
[**validator_unbonding_delegations**](QueryApi.md#validator_unbonding_delegations) | **GET** /cosmos/staking/v1beta1/validators/{validatorAddr}/unbonding_delegations | ValidatorUnbondingDelegations queries unbonding delegations of a validator.
[**validators**](QueryApi.md#validators) | **GET** /cosmos/staking/v1beta1/validators | Validators queries all validators that match the given status.
[**vote**](QueryApi.md#vote) | **GET** /cosmos/gov/v1/proposals/{proposalId}/votes/{voter} | Vote queries voted information based on proposalID, voterAddr.
[**vote_0**](QueryApi.md#vote_0) | **GET** /cosmos/gov/v1beta1/proposals/{proposalId}/votes/{voter} | Vote queries voted information based on proposalID, voterAddr.
[**vote_by_proposal_voter**](QueryApi.md#vote_by_proposal_voter) | **GET** /cosmos/group/v1/vote_by_proposal_voter/{proposalId}/{voter} | VoteByProposalVoter queries a vote by proposal id and voter.
[**votes**](QueryApi.md#votes) | **GET** /cosmos/gov/v1/proposals/{proposalId}/votes | Votes queries votes of a given proposal.
[**votes_0**](QueryApi.md#votes_0) | **GET** /cosmos/gov/v1beta1/proposals/{proposalId}/votes | Votes queries votes of a given proposal.
[**votes_by_proposal**](QueryApi.md#votes_by_proposal) | **GET** /cosmos/group/v1/votes_by_proposal/{proposalId} | VotesByProposal queries a vote by proposal.
[**votes_by_voter**](QueryApi.md#votes_by_voter) | **GET** /cosmos/group/v1/votes_by_voter/{voter} | VotesByVoter queries a vote by voter.



## account

> crate::models::CosmosPeriodAuthPeriodV1beta1PeriodQueryAccountResponse account(address)
Account returns account details based on address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | address defines the address to query for. | [required] |

### Return type

[**crate::models::CosmosPeriodAuthPeriodV1beta1PeriodQueryAccountResponse**](cosmos.auth.v1beta1.QueryAccountResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_0

> crate::models::EthermintPeriodEvmPeriodV1PeriodQueryAccountResponse account_0(address)
Account queries an Ethereum account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | address is the ethereum hex address to query the account for. | [required] |

### Return type

[**crate::models::EthermintPeriodEvmPeriodV1PeriodQueryAccountResponse**](ethermint.evm.v1.QueryAccountResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_address_by_id

> crate::models::CosmosPeriodAuthPeriodV1beta1PeriodQueryAccountAddressByIdResponse account_address_by_id(id)
AccountAddressByID returns account address based on account number.

Since: cosmos-sdk 0.46.2

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | id is the account number of the address to be queried. This field should have been an uint64 (like all account numbers), and will be updated to uint64 in a future version of the auth query. | [required] |

### Return type

[**crate::models::CosmosPeriodAuthPeriodV1beta1PeriodQueryAccountAddressByIdResponse**](cosmos.auth.v1beta1.QueryAccountAddressByIDResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts

> crate::models::CosmosPeriodAuthPeriodV1beta1PeriodQueryAccountsResponse accounts(pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
Accounts returns all the existing accounts

Since: cosmos-sdk 0.43

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodAuthPeriodV1beta1PeriodQueryAccountsResponse**](cosmos.auth.v1beta1.QueryAccountsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## address_bytes_to_string

> crate::models::CosmosPeriodAuthPeriodV1beta1PeriodAddressBytesToStringResponse address_bytes_to_string(address_bytes)
AddressBytesToString converts Account Address bytes to string

Since: cosmos-sdk 0.46

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address_bytes** | **String** |  | [required] |

### Return type

[**crate::models::CosmosPeriodAuthPeriodV1beta1PeriodAddressBytesToStringResponse**](cosmos.auth.v1beta1.AddressBytesToStringResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## address_string_to_bytes

> crate::models::CosmosPeriodAuthPeriodV1beta1PeriodAddressStringToBytesResponse address_string_to_bytes(address_string)
AddressStringToBytes converts Address string to bytes

Since: cosmos-sdk 0.46

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address_string** | **String** |  | [required] |

### Return type

[**crate::models::CosmosPeriodAuthPeriodV1beta1PeriodAddressStringToBytesResponse**](cosmos.auth.v1beta1.AddressStringToBytesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## all_balances

> crate::models::CosmosPeriodBankPeriodV1beta1PeriodQueryAllBalancesResponse all_balances(address, pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
AllBalances queries the balance of all coins for a single account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | address is the address to query balances for. | [required] |
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodBankPeriodV1beta1PeriodQueryAllBalancesResponse**](cosmos.bank.v1beta1.QueryAllBalancesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## all_evidence

> crate::models::CosmosPeriodEvidencePeriodV1beta1PeriodQueryAllEvidenceResponse all_evidence(pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
AllEvidence queries all evidence.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodEvidencePeriodV1beta1PeriodQueryAllEvidenceResponse**](cosmos.evidence.v1beta1.QueryAllEvidenceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allowance

> crate::models::CosmosPeriodFeegrantPeriodV1beta1PeriodQueryAllowanceResponse allowance(granter, grantee)
Allowance returns fee granted to the grantee by the granter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**granter** | **String** | granter is the address of the user granting an allowance of their funds. | [required] |
**grantee** | **String** | grantee is the address of the user being granted an allowance of another user's funds. | [required] |

### Return type

[**crate::models::CosmosPeriodFeegrantPeriodV1beta1PeriodQueryAllowanceResponse**](cosmos.feegrant.v1beta1.QueryAllowanceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allowances

> crate::models::CosmosPeriodFeegrantPeriodV1beta1PeriodQueryAllowancesResponse allowances(grantee, pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
Allowances returns all the grants for address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**grantee** | **String** |  | [required] |
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodFeegrantPeriodV1beta1PeriodQueryAllowancesResponse**](cosmos.feegrant.v1beta1.QueryAllowancesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## allowances_by_granter

> crate::models::CosmosPeriodFeegrantPeriodV1beta1PeriodQueryAllowancesByGranterResponse allowances_by_granter(granter, pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
AllowancesByGranter returns all the grants given by an address

Since: cosmos-sdk 0.46

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**granter** | **String** |  | [required] |
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodFeegrantPeriodV1beta1PeriodQueryAllowancesByGranterResponse**](cosmos.feegrant.v1beta1.QueryAllowancesByGranterResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## annual_provisions

> crate::models::CosmosPeriodMintPeriodV1beta1PeriodQueryAnnualProvisionsResponse annual_provisions()
AnnualProvisions current minting annual provisions value.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CosmosPeriodMintPeriodV1beta1PeriodQueryAnnualProvisionsResponse**](cosmos.mint.v1beta1.QueryAnnualProvisionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## applied_plan

> crate::models::CosmosPeriodUpgradePeriodV1beta1PeriodQueryAppliedPlanResponse applied_plan(name)
AppliedPlan queries a previously applied upgrade plan by its name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | name is the name of the applied plan to query for. | [required] |

### Return type

[**crate::models::CosmosPeriodUpgradePeriodV1beta1PeriodQueryAppliedPlanResponse**](cosmos.upgrade.v1beta1.QueryAppliedPlanResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## authority

> crate::models::CosmosPeriodUpgradePeriodV1beta1PeriodQueryAuthorityResponse authority()
Returns the account with authority to conduct upgrades

Since: cosmos-sdk 0.46

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CosmosPeriodUpgradePeriodV1beta1PeriodQueryAuthorityResponse**](cosmos.upgrade.v1beta1.QueryAuthorityResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## balance

> crate::models::CosmosPeriodBankPeriodV1beta1PeriodQueryBalanceResponse balance(address, denom)
Balance queries the balance of a single coin for a single account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | address is the address to query balances for. | [required] |
**denom** | Option<**String**> | denom is the coin denom to query balances for. |  |

### Return type

[**crate::models::CosmosPeriodBankPeriodV1beta1PeriodQueryBalanceResponse**](cosmos.bank.v1beta1.QueryBalanceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## balance_0

> crate::models::CosmosPeriodNftPeriodV1beta1PeriodQueryBalanceResponse balance_0(owner, class_id)
Balance queries the number of NFTs of a given class owned by the owner, same as balanceOf in ERC721

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** |  | [required] |
**class_id** | **String** |  | [required] |

### Return type

[**crate::models::CosmosPeriodNftPeriodV1beta1PeriodQueryBalanceResponse**](cosmos.nft.v1beta1.QueryBalanceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## balance_1

> crate::models::EthermintPeriodEvmPeriodV1PeriodQueryBalanceResponse balance_1(address)
Balance queries the balance of a the EVM denomination for a single EthAccount.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | address is the ethereum hex address to query the balance for. | [required] |

### Return type

[**crate::models::EthermintPeriodEvmPeriodV1PeriodQueryBalanceResponse**](ethermint.evm.v1.QueryBalanceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## balances

> crate::models::HaqqPeriodVestingPeriodV1PeriodQueryBalancesResponse balances(address)
Balances retrieves the unvested, vested and locked tokens for a vesting account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | address of the clawback vesting account | [required] |

### Return type

[**crate::models::HaqqPeriodVestingPeriodV1PeriodQueryBalancesResponse**](haqq.vesting.v1.QueryBalancesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## base_fee

> crate::models::EthermintPeriodEvmPeriodV1PeriodQueryBaseFeeResponse base_fee()
BaseFee queries the base fee of the parent block of the current block, it's similar to feemarket module's method, but also checks london hardfork status.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::EthermintPeriodEvmPeriodV1PeriodQueryBaseFeeResponse**](ethermint.evm.v1.QueryBaseFeeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## base_fee_0

> crate::models::EthermintPeriodFeemarketPeriodV1PeriodQueryBaseFeeResponse base_fee_0()
BaseFee queries the base fee of the parent block of the current block.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::EthermintPeriodFeemarketPeriodV1PeriodQueryBaseFeeResponse**](ethermint.feemarket.v1.QueryBaseFeeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bech32_prefix

> crate::models::CosmosPeriodAuthPeriodV1beta1PeriodBech32PrefixResponse bech32_prefix()
Bech32Prefix queries bech32Prefix

Since: cosmos-sdk 0.46

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CosmosPeriodAuthPeriodV1beta1PeriodBech32PrefixResponse**](cosmos.auth.v1beta1.Bech32PrefixResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## block_gas

> crate::models::EthermintPeriodFeemarketPeriodV1PeriodQueryBlockGasResponse block_gas()
BlockGas queries the gas used at a given block height

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::EthermintPeriodFeemarketPeriodV1PeriodQueryBlockGasResponse**](ethermint.feemarket.v1.QueryBlockGasResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## class

> crate::models::CosmosPeriodNftPeriodV1beta1PeriodQueryClassResponse class(class_id)
Class queries an NFT class based on its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**class_id** | **String** |  | [required] |

### Return type

[**crate::models::CosmosPeriodNftPeriodV1beta1PeriodQueryClassResponse**](cosmos.nft.v1beta1.QueryClassResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## classes

> crate::models::CosmosPeriodNftPeriodV1beta1PeriodQueryClassesResponse classes(pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
Classes queries all NFT classes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodNftPeriodV1beta1PeriodQueryClassesResponse**](cosmos.nft.v1beta1.QueryClassesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## code

> crate::models::EthermintPeriodEvmPeriodV1PeriodQueryCodeResponse code(address)
Code queries the balance of all coins for a single account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | address is the ethereum hex address to query the code for. | [required] |

### Return type

[**crate::models::EthermintPeriodEvmPeriodV1PeriodQueryCodeResponse**](ethermint.evm.v1.QueryCodeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## community_pool

> crate::models::CosmosPeriodDistributionPeriodV1beta1PeriodQueryCommunityPoolResponse community_pool()
CommunityPool queries the community pool coins.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CosmosPeriodDistributionPeriodV1beta1PeriodQueryCommunityPoolResponse**](cosmos.distribution.v1beta1.QueryCommunityPoolResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cosmos_account

> crate::models::EthermintPeriodEvmPeriodV1PeriodQueryCosmosAccountResponse cosmos_account(address)
CosmosAccount queries an Ethereum account's Cosmos Address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | address is the ethereum hex address to query the account for. | [required] |

### Return type

[**crate::models::EthermintPeriodEvmPeriodV1PeriodQueryCosmosAccountResponse**](ethermint.evm.v1.QueryCosmosAccountResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## current_epoch

> crate::models::EvmosPeriodEpochsPeriodV1PeriodQueryCurrentEpochResponse current_epoch(identifier)
CurrentEpoch provide current epoch of specified identifier

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identifier** | Option<**String**> | identifier of the current epoch |  |

### Return type

[**crate::models::EvmosPeriodEpochsPeriodV1PeriodQueryCurrentEpochResponse**](evmos.epochs.v1.QueryCurrentEpochResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## current_plan

> crate::models::CosmosPeriodUpgradePeriodV1beta1PeriodQueryCurrentPlanResponse current_plan()
CurrentPlan queries the current upgrade plan.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CosmosPeriodUpgradePeriodV1beta1PeriodQueryCurrentPlanResponse**](cosmos.upgrade.v1beta1.QueryCurrentPlanResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delegation

> crate::models::CosmosPeriodStakingPeriodV1beta1PeriodQueryDelegationResponse delegation(validator_addr, delegator_addr)
Delegation queries delegate info for given validator delegator pair.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**validator_addr** | **String** | validator_addr defines the validator address to query for. | [required] |
**delegator_addr** | **String** | delegator_addr defines the delegator address to query for. | [required] |

### Return type

[**crate::models::CosmosPeriodStakingPeriodV1beta1PeriodQueryDelegationResponse**](cosmos.staking.v1beta1.QueryDelegationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delegation_rewards

> crate::models::CosmosPeriodDistributionPeriodV1beta1PeriodQueryDelegationRewardsResponse delegation_rewards(delegator_address, validator_address)
DelegationRewards queries the total rewards accrued by a delegation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delegator_address** | **String** | delegator_address defines the delegator address to query for. | [required] |
**validator_address** | **String** | validator_address defines the validator address to query for. | [required] |

### Return type

[**crate::models::CosmosPeriodDistributionPeriodV1beta1PeriodQueryDelegationRewardsResponse**](cosmos.distribution.v1beta1.QueryDelegationRewardsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delegation_total_rewards

> crate::models::CosmosPeriodDistributionPeriodV1beta1PeriodQueryDelegationTotalRewardsResponse delegation_total_rewards(delegator_address)
DelegationTotalRewards queries the total rewards accrued by a each validator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delegator_address** | **String** | delegator_address defines the delegator address to query for. | [required] |

### Return type

[**crate::models::CosmosPeriodDistributionPeriodV1beta1PeriodQueryDelegationTotalRewardsResponse**](cosmos.distribution.v1beta1.QueryDelegationTotalRewardsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delegator_delegations

> crate::models::CosmosPeriodStakingPeriodV1beta1PeriodQueryDelegatorDelegationsResponse delegator_delegations(delegator_addr, pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
DelegatorDelegations queries all delegations of a given delegator address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delegator_addr** | **String** | delegator_addr defines the delegator address to query for. | [required] |
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodStakingPeriodV1beta1PeriodQueryDelegatorDelegationsResponse**](cosmos.staking.v1beta1.QueryDelegatorDelegationsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delegator_unbonding_delegations

> crate::models::CosmosPeriodStakingPeriodV1beta1PeriodQueryDelegatorUnbondingDelegationsResponse delegator_unbonding_delegations(delegator_addr, pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
DelegatorUnbondingDelegations queries all unbonding delegations of a given delegator address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delegator_addr** | **String** | delegator_addr defines the delegator address to query for. | [required] |
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodStakingPeriodV1beta1PeriodQueryDelegatorUnbondingDelegationsResponse**](cosmos.staking.v1beta1.QueryDelegatorUnbondingDelegationsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delegator_validator

> crate::models::CosmosPeriodStakingPeriodV1beta1PeriodQueryDelegatorValidatorResponse delegator_validator(delegator_addr, validator_addr)
DelegatorValidator queries validator info for given delegator validator pair.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delegator_addr** | **String** | delegator_addr defines the delegator address to query for. | [required] |
**validator_addr** | **String** | validator_addr defines the validator address to query for. | [required] |

### Return type

[**crate::models::CosmosPeriodStakingPeriodV1beta1PeriodQueryDelegatorValidatorResponse**](cosmos.staking.v1beta1.QueryDelegatorValidatorResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delegator_validators

> crate::models::CosmosPeriodDistributionPeriodV1beta1PeriodQueryDelegatorValidatorsResponse delegator_validators(delegator_address)
DelegatorValidators queries the validators of a delegator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delegator_address** | **String** | delegator_address defines the delegator address to query for. | [required] |

### Return type

[**crate::models::CosmosPeriodDistributionPeriodV1beta1PeriodQueryDelegatorValidatorsResponse**](cosmos.distribution.v1beta1.QueryDelegatorValidatorsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delegator_validators_0

> crate::models::CosmosPeriodStakingPeriodV1beta1PeriodQueryDelegatorValidatorsResponse delegator_validators_0(delegator_addr, pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
DelegatorValidators queries all validators info for given delegator address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delegator_addr** | **String** | delegator_addr defines the delegator address to query for. | [required] |
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodStakingPeriodV1beta1PeriodQueryDelegatorValidatorsResponse**](cosmos.staking.v1beta1.QueryDelegatorValidatorsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delegator_withdraw_address

> crate::models::CosmosPeriodDistributionPeriodV1beta1PeriodQueryDelegatorWithdrawAddressResponse delegator_withdraw_address(delegator_address)
DelegatorWithdrawAddress queries withdraw address of a delegator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delegator_address** | **String** | delegator_address defines the delegator address to query for. | [required] |

### Return type

[**crate::models::CosmosPeriodDistributionPeriodV1beta1PeriodQueryDelegatorWithdrawAddressResponse**](cosmos.distribution.v1beta1.QueryDelegatorWithdrawAddressResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## denom_metadata

> crate::models::CosmosPeriodBankPeriodV1beta1PeriodQueryDenomMetadataResponse denom_metadata(denom)
DenomsMetadata queries the client metadata of a given coin denomination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**denom** | **String** | denom is the coin denom to query the metadata for. | [required] |

### Return type

[**crate::models::CosmosPeriodBankPeriodV1beta1PeriodQueryDenomMetadataResponse**](cosmos.bank.v1beta1.QueryDenomMetadataResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## denom_owners

> crate::models::CosmosPeriodBankPeriodV1beta1PeriodQueryDenomOwnersResponse denom_owners(denom, pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
DenomOwners queries for all account addresses that own a particular token denomination.

Since: cosmos-sdk 0.46

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**denom** | **String** | denom defines the coin denomination to query all account holders for. | [required] |
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodBankPeriodV1beta1PeriodQueryDenomOwnersResponse**](cosmos.bank.v1beta1.QueryDenomOwnersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## denoms_metadata

> crate::models::CosmosPeriodBankPeriodV1beta1PeriodQueryDenomsMetadataResponse denoms_metadata(pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
DenomsMetadata queries the client metadata for all registered coin denominations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodBankPeriodV1beta1PeriodQueryDenomsMetadataResponse**](cosmos.bank.v1beta1.QueryDenomsMetadataResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deposit

> crate::models::CosmosPeriodGovPeriodV1PeriodQueryDepositResponse deposit(proposal_id, depositor)
Deposit queries single deposit information based proposalID, depositAddr.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_id** | **String** | proposal_id defines the unique id of the proposal. | [required] |
**depositor** | **String** | depositor defines the deposit addresses from the proposals. | [required] |

### Return type

[**crate::models::CosmosPeriodGovPeriodV1PeriodQueryDepositResponse**](cosmos.gov.v1.QueryDepositResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deposit_0

> crate::models::CosmosPeriodGovPeriodV1beta1PeriodQueryDepositResponse deposit_0(proposal_id, depositor)
Deposit queries single deposit information based proposalID, depositAddr.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_id** | **String** | proposal_id defines the unique id of the proposal. | [required] |
**depositor** | **String** | depositor defines the deposit addresses from the proposals. | [required] |

### Return type

[**crate::models::CosmosPeriodGovPeriodV1beta1PeriodQueryDepositResponse**](cosmos.gov.v1beta1.QueryDepositResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deposits

> crate::models::CosmosPeriodGovPeriodV1PeriodQueryDepositsResponse deposits(proposal_id, pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
Deposits queries all deposits of a single proposal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_id** | **String** | proposal_id defines the unique id of the proposal. | [required] |
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodGovPeriodV1PeriodQueryDepositsResponse**](cosmos.gov.v1.QueryDepositsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deposits_0

> crate::models::CosmosPeriodGovPeriodV1beta1PeriodQueryDepositsResponse deposits_0(proposal_id, pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
Deposits queries all deposits of a single proposal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_id** | **String** | proposal_id defines the unique id of the proposal. | [required] |
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodGovPeriodV1beta1PeriodQueryDepositsResponse**](cosmos.gov.v1beta1.QueryDepositsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## epoch_infos

> crate::models::EvmosPeriodEpochsPeriodV1PeriodQueryEpochsInfoResponse epoch_infos(pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
EpochInfos provide running epochInfos

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::EvmosPeriodEpochsPeriodV1PeriodQueryEpochsInfoResponse**](evmos.epochs.v1.QueryEpochsInfoResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## era

> crate::models::HaqqPeriodCoinomicsPeriodV1PeriodQueryEraResponse era()
Era retrieves current era.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::HaqqPeriodCoinomicsPeriodV1PeriodQueryEraResponse**](haqq.coinomics.v1.QueryEraResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## era_closing_supply

> crate::models::HaqqPeriodCoinomicsPeriodV1PeriodQueryEraClosingSupplyResponse era_closing_supply()
EraTargetSupply retrieves current era target supply.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::HaqqPeriodCoinomicsPeriodV1PeriodQueryEraClosingSupplyResponse**](haqq.coinomics.v1.QueryEraClosingSupplyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## estimate_gas

> crate::models::EthermintPeriodEvmPeriodV1PeriodEstimateGasResponse estimate_gas(args, gas_cap, proposer_address, chain_id)
EstimateGas implements the `eth_estimateGas` rpc api

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**args** | Option<**String**> | args uses the same json format as the json rpc api. |  |
**gas_cap** | Option<**String**> | gas_cap defines the default gas cap to be used |  |
**proposer_address** | Option<**String**> | proposer_address of the requested block in hex format |  |
**chain_id** | Option<**String**> | chain_id is the eip155 chain id parsed from the requested block header |  |

### Return type

[**crate::models::EthermintPeriodEvmPeriodV1PeriodEstimateGasResponse**](ethermint.evm.v1.EstimateGasResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## eth_call

> crate::models::EthermintPeriodEvmPeriodV1PeriodMsgEthereumTxResponse eth_call(args, gas_cap, proposer_address, chain_id)
EthCall implements the `eth_call` rpc api

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**args** | Option<**String**> | args uses the same json format as the json rpc api. |  |
**gas_cap** | Option<**String**> | gas_cap defines the default gas cap to be used |  |
**proposer_address** | Option<**String**> | proposer_address of the requested block in hex format |  |
**chain_id** | Option<**String**> | chain_id is the eip155 chain id parsed from the requested block header |  |

### Return type

[**crate::models::EthermintPeriodEvmPeriodV1PeriodMsgEthereumTxResponse**](ethermint.evm.v1.MsgEthereumTxResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## evidence

> crate::models::CosmosPeriodEvidencePeriodV1beta1PeriodQueryEvidenceResponse evidence(evidence_hash)
Evidence queries evidence based on evidence hash.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**evidence_hash** | **String** | evidence_hash defines the hash of the requested evidence. | [required] |

### Return type

[**crate::models::CosmosPeriodEvidencePeriodV1beta1PeriodQueryEvidenceResponse**](cosmos.evidence.v1beta1.QueryEvidenceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grantee_grants

> crate::models::CosmosPeriodAuthzPeriodV1beta1PeriodQueryGranteeGrantsResponse grantee_grants(grantee, pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
GranteeGrants returns a list of `GrantAuthorization` by grantee.

Since: cosmos-sdk 0.46

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**grantee** | **String** |  | [required] |
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodAuthzPeriodV1beta1PeriodQueryGranteeGrantsResponse**](cosmos.authz.v1beta1.QueryGranteeGrantsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## granter_grants

> crate::models::CosmosPeriodAuthzPeriodV1beta1PeriodQueryGranterGrantsResponse granter_grants(granter, pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
GranterGrants returns list of `GrantAuthorization`, granted by granter.

Since: cosmos-sdk 0.46

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**granter** | **String** |  | [required] |
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodAuthzPeriodV1beta1PeriodQueryGranterGrantsResponse**](cosmos.authz.v1beta1.QueryGranterGrantsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grants

> crate::models::CosmosPeriodAuthzPeriodV1beta1PeriodQueryGrantsResponse grants(granter, grantee, msg_type_url, pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
Returns list of `Authorization`, granted to the grantee by the granter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**granter** | Option<**String**> |  |  |
**grantee** | Option<**String**> |  |  |
**msg_type_url** | Option<**String**> | Optional, msg_type_url, when set, will query only grants matching given msg type. |  |
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodAuthzPeriodV1beta1PeriodQueryGrantsResponse**](cosmos.authz.v1beta1.QueryGrantsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_info

> crate::models::CosmosPeriodGroupPeriodV1PeriodQueryGroupInfoResponse group_info(group_id)
GroupInfo queries group info based on group id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | group_id is the unique ID of the group. | [required] |

### Return type

[**crate::models::CosmosPeriodGroupPeriodV1PeriodQueryGroupInfoResponse**](cosmos.group.v1.QueryGroupInfoResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_members

> crate::models::CosmosPeriodGroupPeriodV1PeriodQueryGroupMembersResponse group_members(group_id, pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
GroupMembers queries members of a group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | group_id is the unique ID of the group. | [required] |
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodGroupPeriodV1PeriodQueryGroupMembersResponse**](cosmos.group.v1.QueryGroupMembersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_policies_by_admin

> crate::models::CosmosPeriodGroupPeriodV1PeriodQueryGroupPoliciesByAdminResponse group_policies_by_admin(admin, pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
GroupsByAdmin queries group policies by admin address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**admin** | **String** | admin is the admin address of the group policy. | [required] |
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodGroupPeriodV1PeriodQueryGroupPoliciesByAdminResponse**](cosmos.group.v1.QueryGroupPoliciesByAdminResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_policies_by_group

> crate::models::CosmosPeriodGroupPeriodV1PeriodQueryGroupPoliciesByGroupResponse group_policies_by_group(group_id, pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
GroupPoliciesByGroup queries group policies by group id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | group_id is the unique ID of the group policy's group. | [required] |
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodGroupPeriodV1PeriodQueryGroupPoliciesByGroupResponse**](cosmos.group.v1.QueryGroupPoliciesByGroupResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_policy_info

> crate::models::CosmosPeriodGroupPeriodV1PeriodQueryGroupPolicyInfoResponse group_policy_info(address)
GroupPolicyInfo queries group policy info based on account address of group policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | address is the account address of the group policy. | [required] |

### Return type

[**crate::models::CosmosPeriodGroupPeriodV1PeriodQueryGroupPolicyInfoResponse**](cosmos.group.v1.QueryGroupPolicyInfoResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups

> crate::models::CosmosPeriodGroupPeriodV1PeriodQueryGroupsResponse groups(pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
Groups queries all groups in state.

Since: cosmos-sdk 0.47.1

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodGroupPeriodV1PeriodQueryGroupsResponse**](cosmos.group.v1.QueryGroupsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_by_admin

> crate::models::CosmosPeriodGroupPeriodV1PeriodQueryGroupsByAdminResponse groups_by_admin(admin, pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
GroupsByAdmin queries groups by admin address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**admin** | **String** | admin is the account address of a group's admin. | [required] |
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodGroupPeriodV1PeriodQueryGroupsByAdminResponse**](cosmos.group.v1.QueryGroupsByAdminResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_by_member

> crate::models::CosmosPeriodGroupPeriodV1PeriodQueryGroupsByMemberResponse groups_by_member(address, pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
GroupsByMember queries groups by member address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | address is the group member address. | [required] |
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodGroupPeriodV1PeriodQueryGroupsByMemberResponse**](cosmos.group.v1.QueryGroupsByMemberResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## historical_info

> crate::models::CosmosPeriodStakingPeriodV1beta1PeriodQueryHistoricalInfoResponse historical_info(height)
HistoricalInfo queries the historical info for given height.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**height** | **String** | height defines at which height to query the historical info. | [required] |

### Return type

[**crate::models::CosmosPeriodStakingPeriodV1beta1PeriodQueryHistoricalInfoResponse**](cosmos.staking.v1beta1.QueryHistoricalInfoResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## inflation

> crate::models::CosmosPeriodMintPeriodV1beta1PeriodQueryInflationResponse inflation()
Inflation returns the current minting inflation value.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CosmosPeriodMintPeriodV1beta1PeriodQueryInflationResponse**](cosmos.mint.v1beta1.QueryInflationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## inflation_rate

> crate::models::HaqqPeriodCoinomicsPeriodV1PeriodQueryInflationRateResponse inflation_rate()
InflationRate retrieves current era inflation rate.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::HaqqPeriodCoinomicsPeriodV1PeriodQueryInflationRateResponse**](haqq.coinomics.v1.QueryInflationRateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## max_supply

> crate::models::HaqqPeriodCoinomicsPeriodV1PeriodQueryMaxSupplyResponse max_supply()
MaxSupply retrieves total coins of all eras and when mint ended.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::HaqqPeriodCoinomicsPeriodV1PeriodQueryMaxSupplyResponse**](haqq.coinomics.v1.QueryMaxSupplyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## module_account_by_name

> crate::models::CosmosPeriodAuthPeriodV1beta1PeriodQueryModuleAccountByNameResponse module_account_by_name(name)
ModuleAccountByName returns the module account info by module name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::CosmosPeriodAuthPeriodV1beta1PeriodQueryModuleAccountByNameResponse**](cosmos.auth.v1beta1.QueryModuleAccountByNameResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## module_accounts

> crate::models::CosmosPeriodAuthPeriodV1beta1PeriodQueryModuleAccountsResponse module_accounts()
ModuleAccounts returns all the existing module accounts.

Since: cosmos-sdk 0.46

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CosmosPeriodAuthPeriodV1beta1PeriodQueryModuleAccountsResponse**](cosmos.auth.v1beta1.QueryModuleAccountsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## module_versions

> crate::models::CosmosPeriodUpgradePeriodV1beta1PeriodQueryModuleVersionsResponse module_versions(module_name)
ModuleVersions queries the list of module versions from state.

Since: cosmos-sdk 0.43

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**module_name** | Option<**String**> | module_name is a field to query a specific module consensus version from state. Leaving this empty will fetch the full list of module versions from state |  |

### Return type

[**crate::models::CosmosPeriodUpgradePeriodV1beta1PeriodQueryModuleVersionsResponse**](cosmos.upgrade.v1beta1.QueryModuleVersionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## n_ft

> crate::models::CosmosPeriodNftPeriodV1beta1PeriodQueryNftResponse n_ft(class_id, id)
NFT queries an NFT based on its class and id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**class_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**crate::models::CosmosPeriodNftPeriodV1beta1PeriodQueryNftResponse**](cosmos.nft.v1beta1.QueryNFTResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## n_fts

> crate::models::CosmosPeriodNftPeriodV1beta1PeriodQueryNftsResponse n_fts(class_id, owner, pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
NFTs queries all NFTs of a given class or owner,choose at least one of the two, similar to tokenByIndex in ERC721Enumerable

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**class_id** | Option<**String**> |  |  |
**owner** | Option<**String**> |  |  |
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodNftPeriodV1beta1PeriodQueryNftsResponse**](cosmos.nft.v1beta1.QueryNFTsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## owner

> crate::models::CosmosPeriodNftPeriodV1beta1PeriodQueryOwnerResponse owner(class_id, id)
Owner queries the owner of the NFT based on its class and id, same as ownerOf in ERC721

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**class_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**crate::models::CosmosPeriodNftPeriodV1beta1PeriodQueryOwnerResponse**](cosmos.nft.v1beta1.QueryOwnerResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## params

> crate::models::CosmosPeriodAuthPeriodV1beta1PeriodQueryParamsResponse params()
Params queries all parameters.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CosmosPeriodAuthPeriodV1beta1PeriodQueryParamsResponse**](cosmos.auth.v1beta1.QueryParamsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## params_0

> crate::models::CosmosPeriodBankPeriodV1beta1PeriodQueryParamsResponse params_0()
Params queries the parameters of x/bank module.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CosmosPeriodBankPeriodV1beta1PeriodQueryParamsResponse**](cosmos.bank.v1beta1.QueryParamsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## params_1

> crate::models::CosmosPeriodDistributionPeriodV1beta1PeriodQueryParamsResponse params_1()
Params queries params of the distribution module.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CosmosPeriodDistributionPeriodV1beta1PeriodQueryParamsResponse**](cosmos.distribution.v1beta1.QueryParamsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## params_10

> crate::models::EthermintPeriodFeemarketPeriodV1PeriodQueryParamsResponse params_10()
Params queries the parameters of x/feemarket module.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::EthermintPeriodFeemarketPeriodV1PeriodQueryParamsResponse**](ethermint.feemarket.v1.QueryParamsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## params_11

> crate::models::HaqqPeriodCoinomicsPeriodV1PeriodQueryParamsResponse params_11()
Params retrieves coinomics moudle params.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::HaqqPeriodCoinomicsPeriodV1PeriodQueryParamsResponse**](haqq.coinomics.v1.QueryParamsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## params_2

> crate::models::CosmosPeriodGovPeriodV1PeriodQueryParamsResponse params_2(params_type)
Params queries all parameters of the gov module.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**params_type** | **String** | params_type defines which parameters to query for, can be one of \"voting\", \"tallying\" or \"deposit\". | [required] |

### Return type

[**crate::models::CosmosPeriodGovPeriodV1PeriodQueryParamsResponse**](cosmos.gov.v1.QueryParamsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## params_3

> crate::models::CosmosPeriodGovPeriodV1beta1PeriodQueryParamsResponse params_3(params_type)
Params queries all parameters of the gov module.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**params_type** | **String** | params_type defines which parameters to query for, can be one of \"voting\", \"tallying\" or \"deposit\". | [required] |

### Return type

[**crate::models::CosmosPeriodGovPeriodV1beta1PeriodQueryParamsResponse**](cosmos.gov.v1beta1.QueryParamsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## params_4

> crate::models::CosmosPeriodMintPeriodV1beta1PeriodQueryParamsResponse params_4()
Params returns the total set of minting parameters.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CosmosPeriodMintPeriodV1beta1PeriodQueryParamsResponse**](cosmos.mint.v1beta1.QueryParamsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## params_5

> crate::models::CosmosPeriodParamsPeriodV1beta1PeriodQueryParamsResponse params_5(subspace, key)
Params queries a specific parameter of a module, given its subspace and key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subspace** | Option<**String**> | subspace defines the module to query the parameter for. |  |
**key** | Option<**String**> | key defines the key of the parameter in the subspace. |  |

### Return type

[**crate::models::CosmosPeriodParamsPeriodV1beta1PeriodQueryParamsResponse**](cosmos.params.v1beta1.QueryParamsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## params_6

> crate::models::CosmosPeriodSlashingPeriodV1beta1PeriodQueryParamsResponse params_6()
Params queries the parameters of slashing module

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CosmosPeriodSlashingPeriodV1beta1PeriodQueryParamsResponse**](cosmos.slashing.v1beta1.QueryParamsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## params_7

> crate::models::CosmosPeriodStakingPeriodV1beta1PeriodQueryParamsResponse params_7()
Parameters queries the staking parameters.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CosmosPeriodStakingPeriodV1beta1PeriodQueryParamsResponse**](cosmos.staking.v1beta1.QueryParamsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## params_8

> crate::models::EvmosPeriodErc20PeriodV1PeriodQueryParamsResponse params_8()
Params retrieves the erc20 module params

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::EvmosPeriodErc20PeriodV1PeriodQueryParamsResponse**](evmos.erc20.v1.QueryParamsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## params_9

> crate::models::EthermintPeriodEvmPeriodV1PeriodQueryParamsResponse params_9()
Params queries the parameters of x/evm module.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::EthermintPeriodEvmPeriodV1PeriodQueryParamsResponse**](ethermint.evm.v1.QueryParamsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pool

> crate::models::CosmosPeriodStakingPeriodV1beta1PeriodQueryPoolResponse pool()
Pool queries the pool info.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CosmosPeriodStakingPeriodV1beta1PeriodQueryPoolResponse**](cosmos.staking.v1beta1.QueryPoolResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## proposal

> crate::models::CosmosPeriodGovPeriodV1PeriodQueryProposalResponse proposal(proposal_id)
Proposal queries proposal details based on ProposalID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_id** | **String** | proposal_id defines the unique id of the proposal. | [required] |

### Return type

[**crate::models::CosmosPeriodGovPeriodV1PeriodQueryProposalResponse**](cosmos.gov.v1.QueryProposalResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## proposal_0

> crate::models::CosmosPeriodGovPeriodV1beta1PeriodQueryProposalResponse proposal_0(proposal_id)
Proposal queries proposal details based on ProposalID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_id** | **String** | proposal_id defines the unique id of the proposal. | [required] |

### Return type

[**crate::models::CosmosPeriodGovPeriodV1beta1PeriodQueryProposalResponse**](cosmos.gov.v1beta1.QueryProposalResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## proposal_1

> crate::models::CosmosPeriodGroupPeriodV1PeriodQueryProposalResponse proposal_1(proposal_id)
Proposal queries a proposal based on proposal id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_id** | **String** | proposal_id is the unique ID of a proposal. | [required] |

### Return type

[**crate::models::CosmosPeriodGroupPeriodV1PeriodQueryProposalResponse**](cosmos.group.v1.QueryProposalResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## proposals

> crate::models::CosmosPeriodGovPeriodV1PeriodQueryProposalsResponse proposals(proposal_status, voter, depositor, pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
Proposals queries all proposals based on given status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_status** | Option<**String**> | proposal_status defines the status of the proposals.   - PROPOSAL_STATUS_UNSPECIFIED: PROPOSAL_STATUS_UNSPECIFIED defines the default proposal status.  - PROPOSAL_STATUS_DEPOSIT_PERIOD: PROPOSAL_STATUS_DEPOSIT_PERIOD defines a proposal status during the deposit period.  - PROPOSAL_STATUS_VOTING_PERIOD: PROPOSAL_STATUS_VOTING_PERIOD defines a proposal status during the voting period.  - PROPOSAL_STATUS_PASSED: PROPOSAL_STATUS_PASSED defines a proposal status of a proposal that has passed.  - PROPOSAL_STATUS_REJECTED: PROPOSAL_STATUS_REJECTED defines a proposal status of a proposal that has been rejected.  - PROPOSAL_STATUS_FAILED: PROPOSAL_STATUS_FAILED defines a proposal status of a proposal that has failed. |  |[default to PROPOSAL_STATUS_UNSPECIFIED]
**voter** | Option<**String**> | voter defines the voter address for the proposals. |  |
**depositor** | Option<**String**> | depositor defines the deposit addresses from the proposals. |  |
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodGovPeriodV1PeriodQueryProposalsResponse**](cosmos.gov.v1.QueryProposalsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## proposals_0

> crate::models::CosmosPeriodGovPeriodV1beta1PeriodQueryProposalsResponse proposals_0(proposal_status, voter, depositor, pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
Proposals queries all proposals based on given status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_status** | Option<**String**> | proposal_status defines the status of the proposals.   - PROPOSAL_STATUS_UNSPECIFIED: PROPOSAL_STATUS_UNSPECIFIED defines the default proposal status.  - PROPOSAL_STATUS_DEPOSIT_PERIOD: PROPOSAL_STATUS_DEPOSIT_PERIOD defines a proposal status during the deposit period.  - PROPOSAL_STATUS_VOTING_PERIOD: PROPOSAL_STATUS_VOTING_PERIOD defines a proposal status during the voting period.  - PROPOSAL_STATUS_PASSED: PROPOSAL_STATUS_PASSED defines a proposal status of a proposal that has passed.  - PROPOSAL_STATUS_REJECTED: PROPOSAL_STATUS_REJECTED defines a proposal status of a proposal that has been rejected.  - PROPOSAL_STATUS_FAILED: PROPOSAL_STATUS_FAILED defines a proposal status of a proposal that has failed. |  |[default to PROPOSAL_STATUS_UNSPECIFIED]
**voter** | Option<**String**> | voter defines the voter address for the proposals. |  |
**depositor** | Option<**String**> | depositor defines the deposit addresses from the proposals. |  |
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodGovPeriodV1beta1PeriodQueryProposalsResponse**](cosmos.gov.v1beta1.QueryProposalsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## proposals_by_group_policy

> crate::models::CosmosPeriodGroupPeriodV1PeriodQueryProposalsByGroupPolicyResponse proposals_by_group_policy(address, pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
ProposalsByGroupPolicy queries proposals based on account address of group policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | address is the account address of the group policy related to proposals. | [required] |
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodGroupPeriodV1PeriodQueryProposalsByGroupPolicyResponse**](cosmos.group.v1.QueryProposalsByGroupPolicyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## redelegations

> crate::models::CosmosPeriodStakingPeriodV1beta1PeriodQueryRedelegationsResponse redelegations(delegator_addr, src_validator_addr, dst_validator_addr, pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
Redelegations queries redelegations of given address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delegator_addr** | **String** | delegator_addr defines the delegator address to query for. | [required] |
**src_validator_addr** | Option<**String**> | src_validator_addr defines the validator address to redelegate from. |  |
**dst_validator_addr** | Option<**String**> | dst_validator_addr defines the validator address to redelegate to. |  |
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodStakingPeriodV1beta1PeriodQueryRedelegationsResponse**](cosmos.staking.v1beta1.QueryRedelegationsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## signing_info

> crate::models::CosmosPeriodSlashingPeriodV1beta1PeriodQuerySigningInfoResponse signing_info(cons_address)
SigningInfo queries the signing info of given cons address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cons_address** | **String** | cons_address is the address to query signing info of | [required] |

### Return type

[**crate::models::CosmosPeriodSlashingPeriodV1beta1PeriodQuerySigningInfoResponse**](cosmos.slashing.v1beta1.QuerySigningInfoResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## signing_infos

> crate::models::CosmosPeriodSlashingPeriodV1beta1PeriodQuerySigningInfosResponse signing_infos(pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
SigningInfos queries signing info of all validators

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodSlashingPeriodV1beta1PeriodQuerySigningInfosResponse**](cosmos.slashing.v1beta1.QuerySigningInfosResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## spendable_balances

> crate::models::CosmosPeriodBankPeriodV1beta1PeriodQuerySpendableBalancesResponse spendable_balances(address, pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
SpendableBalances queries the spenable balance of all coins for a single account.

Since: cosmos-sdk 0.46

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | address is the address to query spendable balances for. | [required] |
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodBankPeriodV1beta1PeriodQuerySpendableBalancesResponse**](cosmos.bank.v1beta1.QuerySpendableBalancesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage

> crate::models::EthermintPeriodEvmPeriodV1PeriodQueryStorageResponse storage(address, key)
Storage queries the balance of all coins for a single account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | address is the ethereum hex address to query the storage state for. | [required] |
**key** | **String** | key defines the key of the storage state | [required] |

### Return type

[**crate::models::EthermintPeriodEvmPeriodV1PeriodQueryStorageResponse**](ethermint.evm.v1.QueryStorageResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subspaces

> crate::models::CosmosPeriodParamsPeriodV1beta1PeriodQuerySubspacesResponse subspaces()
Subspaces queries for all registered subspaces and all keys for a subspace.

Since: cosmos-sdk 0.46

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CosmosPeriodParamsPeriodV1beta1PeriodQuerySubspacesResponse**](cosmos.params.v1beta1.QuerySubspacesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## supply

> crate::models::CosmosPeriodNftPeriodV1beta1PeriodQuerySupplyResponse supply(class_id)
Supply queries the number of NFTs from the given class, same as totalSupply of ERC721.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**class_id** | **String** |  | [required] |

### Return type

[**crate::models::CosmosPeriodNftPeriodV1beta1PeriodQuerySupplyResponse**](cosmos.nft.v1beta1.QuerySupplyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## supply_of

> crate::models::CosmosPeriodBankPeriodV1beta1PeriodQuerySupplyOfResponse supply_of(denom)
SupplyOf queries the supply of a single coin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**denom** | Option<**String**> | denom is the coin denom to query balances for. |  |

### Return type

[**crate::models::CosmosPeriodBankPeriodV1beta1PeriodQuerySupplyOfResponse**](cosmos.bank.v1beta1.QuerySupplyOfResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tally_result

> crate::models::CosmosPeriodGovPeriodV1PeriodQueryTallyResultResponse tally_result(proposal_id)
TallyResult queries the tally of a proposal vote.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_id** | **String** | proposal_id defines the unique id of the proposal. | [required] |

### Return type

[**crate::models::CosmosPeriodGovPeriodV1PeriodQueryTallyResultResponse**](cosmos.gov.v1.QueryTallyResultResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tally_result_0

> crate::models::CosmosPeriodGovPeriodV1beta1PeriodQueryTallyResultResponse tally_result_0(proposal_id)
TallyResult queries the tally of a proposal vote.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_id** | **String** | proposal_id defines the unique id of the proposal. | [required] |

### Return type

[**crate::models::CosmosPeriodGovPeriodV1beta1PeriodQueryTallyResultResponse**](cosmos.gov.v1beta1.QueryTallyResultResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tally_result_1

> crate::models::CosmosPeriodGroupPeriodV1PeriodQueryTallyResultResponse tally_result_1(proposal_id)
TallyResult returns the tally result of a proposal. If the proposal is still in voting period, then this query computes the current tally state, which might not be final. On the other hand, if the proposal is final, then it simply returns the `final_tally_result` state stored in the proposal itself.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_id** | **String** | proposal_id is the unique id of a proposal. | [required] |

### Return type

[**crate::models::CosmosPeriodGroupPeriodV1PeriodQueryTallyResultResponse**](cosmos.group.v1.QueryTallyResultResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## token_pair

> crate::models::EvmosPeriodErc20PeriodV1PeriodQueryTokenPairResponse token_pair(token)
TokenPair retrieves a registered token pair

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | token identifier can be either the hex contract address of the ERC20 or the Cosmos base denomination | [required] |

### Return type

[**crate::models::EvmosPeriodErc20PeriodV1PeriodQueryTokenPairResponse**](evmos.erc20.v1.QueryTokenPairResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## token_pairs

> crate::models::EvmosPeriodErc20PeriodV1PeriodQueryTokenPairsResponse token_pairs(pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
TokenPairs retrieves registered token pairs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::EvmosPeriodErc20PeriodV1PeriodQueryTokenPairsResponse**](evmos.erc20.v1.QueryTokenPairsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## total_locked

> crate::models::HaqqPeriodVestingPeriodV1PeriodQueryTotalLockedResponse total_locked()
TotalLocked retrieves the total summary of all unvested, vested and locked tokens

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::HaqqPeriodVestingPeriodV1PeriodQueryTotalLockedResponse**](haqq.vesting.v1.QueryTotalLockedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## total_supply

> crate::models::CosmosPeriodBankPeriodV1beta1PeriodQueryTotalSupplyResponse total_supply(pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
TotalSupply queries the total supply of all coins.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodBankPeriodV1beta1PeriodQueryTotalSupplyResponse**](cosmos.bank.v1beta1.QueryTotalSupplyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trace_block

> crate::models::EthermintPeriodEvmPeriodV1PeriodQueryTraceBlockResponse trace_block(trace_config_period_tracer, trace_config_period_timeout, trace_config_period_reexec, trace_config_period_disable_stack, trace_config_period_disable_storage, trace_config_period_debug, trace_config_period_limit, trace_config_period_overrides_period_homestead_block, trace_config_period_overrides_period_dao_fork_block, trace_config_period_overrides_period_dao_fork_support, trace_config_period_overrides_period_eip150_block, trace_config_period_overrides_period_eip150_hash, trace_config_period_overrides_period_eip155_block, trace_config_period_overrides_period_eip158_block, trace_config_period_overrides_period_byzantium_block, trace_config_period_overrides_period_constantinople_block, trace_config_period_overrides_period_petersburg_block, trace_config_period_overrides_period_istanbul_block, trace_config_period_overrides_period_muir_glacier_block, trace_config_period_overrides_period_berlin_block, trace_config_period_overrides_period_london_block, trace_config_period_overrides_period_arrow_glacier_block, trace_config_period_overrides_period_gray_glacier_block, trace_config_period_overrides_period_merge_netsplit_block, trace_config_period_overrides_period_shanghai_block, trace_config_period_overrides_period_cancun_block, trace_config_period_enable_memory, trace_config_period_enable_return_data, trace_config_period_tracer_json_config, block_number, block_hash, block_time, proposer_address, chain_id)
TraceBlock implements the `debug_traceBlockByNumber` and `debug_traceBlockByHash` rpc api

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trace_config_period_tracer** | Option<**String**> | tracer is a custom javascript tracer |  |
**trace_config_period_timeout** | Option<**String**> | timeout overrides the default timeout of 5 seconds for JavaScript-based tracing calls |  |
**trace_config_period_reexec** | Option<**String**> | reexec defines the number of blocks the tracer is willing to go back |  |
**trace_config_period_disable_stack** | Option<**bool**> | disable_stack switches stack capture |  |
**trace_config_period_disable_storage** | Option<**bool**> | disable_storage switches storage capture |  |
**trace_config_period_debug** | Option<**bool**> | debug can be used to print output during capture end |  |
**trace_config_period_limit** | Option<**i32**> | limit defines the maximum length of output, but zero means unlimited |  |
**trace_config_period_overrides_period_homestead_block** | Option<**String**> | homestead_block switch (nil no fork, 0 = already homestead) |  |
**trace_config_period_overrides_period_dao_fork_block** | Option<**String**> | dao_fork_block corresponds to TheDAO hard-fork switch block (nil no fork) |  |
**trace_config_period_overrides_period_dao_fork_support** | Option<**bool**> | dao_fork_support defines whether the nodes supports or opposes the DAO hard-fork |  |
**trace_config_period_overrides_period_eip150_block** | Option<**String**> | eip150_block: EIP150 implements the Gas price changes (https://github.com/ethereum/EIPs/issues/150) EIP150 HF block (nil no fork) |  |
**trace_config_period_overrides_period_eip150_hash** | Option<**String**> | eip150_hash: EIP150 HF hash (needed for header only clients as only gas pricing changed) |  |
**trace_config_period_overrides_period_eip155_block** | Option<**String**> | eip155_block: EIP155Block HF block |  |
**trace_config_period_overrides_period_eip158_block** | Option<**String**> | eip158_block: EIP158 HF block |  |
**trace_config_period_overrides_period_byzantium_block** | Option<**String**> | byzantium_block: Byzantium switch block (nil no fork, 0 = already on byzantium) |  |
**trace_config_period_overrides_period_constantinople_block** | Option<**String**> | constantinople_block: Constantinople switch block (nil no fork, 0 = already activated) |  |
**trace_config_period_overrides_period_petersburg_block** | Option<**String**> | petersburg_block: Petersburg switch block (nil same as Constantinople) |  |
**trace_config_period_overrides_period_istanbul_block** | Option<**String**> | istanbul_block: Istanbul switch block (nil no fork, 0 = already on istanbul) |  |
**trace_config_period_overrides_period_muir_glacier_block** | Option<**String**> | muir_glacier_block: Eip-2384 (bomb delay) switch block (nil no fork, 0 = already activated) |  |
**trace_config_period_overrides_period_berlin_block** | Option<**String**> | berlin_block: Berlin switch block (nil = no fork, 0 = already on berlin) |  |
**trace_config_period_overrides_period_london_block** | Option<**String**> | london_block: London switch block (nil = no fork, 0 = already on london) |  |
**trace_config_period_overrides_period_arrow_glacier_block** | Option<**String**> | arrow_glacier_block: Eip-4345 (bomb delay) switch block (nil = no fork, 0 = already activated) |  |
**trace_config_period_overrides_period_gray_glacier_block** | Option<**String**> | gray_glacier_block: EIP-5133 (bomb delay) switch block (nil = no fork, 0 = already activated) |  |
**trace_config_period_overrides_period_merge_netsplit_block** | Option<**String**> | merge_netsplit_block: Virtual fork after The Merge to use as a network splitter |  |
**trace_config_period_overrides_period_shanghai_block** | Option<**String**> | shanghai_block switch block (nil = no fork, 0 = already on shanghai) |  |
**trace_config_period_overrides_period_cancun_block** | Option<**String**> | cancun_block switch block (nil = no fork, 0 = already on cancun) |  |
**trace_config_period_enable_memory** | Option<**bool**> | enable_memory switches memory capture |  |
**trace_config_period_enable_return_data** | Option<**bool**> | enable_return_data switches the capture of return data |  |
**trace_config_period_tracer_json_config** | Option<**String**> | tracer_json_config configures the tracer using a JSON string |  |
**block_number** | Option<**String**> | block_number of the traced block |  |
**block_hash** | Option<**String**> | block_hash (hex) of the traced block |  |
**block_time** | Option<**String**> | block_time of the traced block |  |
**proposer_address** | Option<**String**> | proposer_address is the address of the requested block |  |
**chain_id** | Option<**String**> | chain_id is the eip155 chain id parsed from the requested block header |  |

### Return type

[**crate::models::EthermintPeriodEvmPeriodV1PeriodQueryTraceBlockResponse**](ethermint.evm.v1.QueryTraceBlockResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trace_tx

> crate::models::EthermintPeriodEvmPeriodV1PeriodQueryTraceTxResponse trace_tx(msg_period_data_period_type_url, msg_period_data_period_value, msg_period_size, msg_period_hash, msg_period_from, trace_config_period_tracer, trace_config_period_timeout, trace_config_period_reexec, trace_config_period_disable_stack, trace_config_period_disable_storage, trace_config_period_debug, trace_config_period_limit, trace_config_period_overrides_period_homestead_block, trace_config_period_overrides_period_dao_fork_block, trace_config_period_overrides_period_dao_fork_support, trace_config_period_overrides_period_eip150_block, trace_config_period_overrides_period_eip150_hash, trace_config_period_overrides_period_eip155_block, trace_config_period_overrides_period_eip158_block, trace_config_period_overrides_period_byzantium_block, trace_config_period_overrides_period_constantinople_block, trace_config_period_overrides_period_petersburg_block, trace_config_period_overrides_period_istanbul_block, trace_config_period_overrides_period_muir_glacier_block, trace_config_period_overrides_period_berlin_block, trace_config_period_overrides_period_london_block, trace_config_period_overrides_period_arrow_glacier_block, trace_config_period_overrides_period_gray_glacier_block, trace_config_period_overrides_period_merge_netsplit_block, trace_config_period_overrides_period_shanghai_block, trace_config_period_overrides_period_cancun_block, trace_config_period_enable_memory, trace_config_period_enable_return_data, trace_config_period_tracer_json_config, block_number, block_hash, block_time, proposer_address, chain_id)
TraceTx implements the `debug_traceTransaction` rpc api

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**msg_period_data_period_type_url** | Option<**String**> | A URL/resource name that uniquely identifies the type of the serialized protocol buffer message. This string must contain at least one \"/\" character. The last segment of the URL's path must represent the fully qualified name of the type (as in `path/google.protobuf.Duration`). The name should be in a canonical form (e.g., leading \".\" is not accepted).  In practice, teams usually precompile into the binary all types that they expect it to use in the context of Any. However, for URLs which use the scheme `http`, `https`, or no scheme, one can optionally set up a type server that maps type URLs to message definitions as follows:  * If no scheme is provided, `https` is assumed. * An HTTP GET on the URL must yield a [google.protobuf.Type][]   value in binary format, or produce an error. * Applications are allowed to cache lookup results based on the   URL, or have them precompiled into a binary to avoid any   lookup. Therefore, binary compatibility needs to be preserved   on changes to types. (Use versioned type names to manage   breaking changes.)  Note: this functionality is not currently available in the official protobuf release, and it is not used for type URLs beginning with type.googleapis.com.  Schemes other than `http`, `https` (or the empty scheme) might be used with implementation specific semantics. |  |
**msg_period_data_period_value** | Option<**String**> | Must be a valid serialized protocol buffer of the above specified type. |  |
**msg_period_size** | Option<**f64**> | size is the encoded storage size of the transaction (DEPRECATED) |  |
**msg_period_hash** | Option<**String**> | hash of the transaction in hex format |  |
**msg_period_from** | Option<**String**> | from is the ethereum signer address in hex format. This address value is checked against the address derived from the signature (V, R, S) using the secp256k1 elliptic curve |  |
**trace_config_period_tracer** | Option<**String**> | tracer is a custom javascript tracer |  |
**trace_config_period_timeout** | Option<**String**> | timeout overrides the default timeout of 5 seconds for JavaScript-based tracing calls |  |
**trace_config_period_reexec** | Option<**String**> | reexec defines the number of blocks the tracer is willing to go back |  |
**trace_config_period_disable_stack** | Option<**bool**> | disable_stack switches stack capture |  |
**trace_config_period_disable_storage** | Option<**bool**> | disable_storage switches storage capture |  |
**trace_config_period_debug** | Option<**bool**> | debug can be used to print output during capture end |  |
**trace_config_period_limit** | Option<**i32**> | limit defines the maximum length of output, but zero means unlimited |  |
**trace_config_period_overrides_period_homestead_block** | Option<**String**> | homestead_block switch (nil no fork, 0 = already homestead) |  |
**trace_config_period_overrides_period_dao_fork_block** | Option<**String**> | dao_fork_block corresponds to TheDAO hard-fork switch block (nil no fork) |  |
**trace_config_period_overrides_period_dao_fork_support** | Option<**bool**> | dao_fork_support defines whether the nodes supports or opposes the DAO hard-fork |  |
**trace_config_period_overrides_period_eip150_block** | Option<**String**> | eip150_block: EIP150 implements the Gas price changes (https://github.com/ethereum/EIPs/issues/150) EIP150 HF block (nil no fork) |  |
**trace_config_period_overrides_period_eip150_hash** | Option<**String**> | eip150_hash: EIP150 HF hash (needed for header only clients as only gas pricing changed) |  |
**trace_config_period_overrides_period_eip155_block** | Option<**String**> | eip155_block: EIP155Block HF block |  |
**trace_config_period_overrides_period_eip158_block** | Option<**String**> | eip158_block: EIP158 HF block |  |
**trace_config_period_overrides_period_byzantium_block** | Option<**String**> | byzantium_block: Byzantium switch block (nil no fork, 0 = already on byzantium) |  |
**trace_config_period_overrides_period_constantinople_block** | Option<**String**> | constantinople_block: Constantinople switch block (nil no fork, 0 = already activated) |  |
**trace_config_period_overrides_period_petersburg_block** | Option<**String**> | petersburg_block: Petersburg switch block (nil same as Constantinople) |  |
**trace_config_period_overrides_period_istanbul_block** | Option<**String**> | istanbul_block: Istanbul switch block (nil no fork, 0 = already on istanbul) |  |
**trace_config_period_overrides_period_muir_glacier_block** | Option<**String**> | muir_glacier_block: Eip-2384 (bomb delay) switch block (nil no fork, 0 = already activated) |  |
**trace_config_period_overrides_period_berlin_block** | Option<**String**> | berlin_block: Berlin switch block (nil = no fork, 0 = already on berlin) |  |
**trace_config_period_overrides_period_london_block** | Option<**String**> | london_block: London switch block (nil = no fork, 0 = already on london) |  |
**trace_config_period_overrides_period_arrow_glacier_block** | Option<**String**> | arrow_glacier_block: Eip-4345 (bomb delay) switch block (nil = no fork, 0 = already activated) |  |
**trace_config_period_overrides_period_gray_glacier_block** | Option<**String**> | gray_glacier_block: EIP-5133 (bomb delay) switch block (nil = no fork, 0 = already activated) |  |
**trace_config_period_overrides_period_merge_netsplit_block** | Option<**String**> | merge_netsplit_block: Virtual fork after The Merge to use as a network splitter |  |
**trace_config_period_overrides_period_shanghai_block** | Option<**String**> | shanghai_block switch block (nil = no fork, 0 = already on shanghai) |  |
**trace_config_period_overrides_period_cancun_block** | Option<**String**> | cancun_block switch block (nil = no fork, 0 = already on cancun) |  |
**trace_config_period_enable_memory** | Option<**bool**> | enable_memory switches memory capture |  |
**trace_config_period_enable_return_data** | Option<**bool**> | enable_return_data switches the capture of return data |  |
**trace_config_period_tracer_json_config** | Option<**String**> | tracer_json_config configures the tracer using a JSON string |  |
**block_number** | Option<**String**> | block_number of requested transaction |  |
**block_hash** | Option<**String**> | block_hash of requested transaction |  |
**block_time** | Option<**String**> | block_time of requested transaction |  |
**proposer_address** | Option<**String**> | proposer_address is the proposer of the requested block |  |
**chain_id** | Option<**String**> | chain_id is the the eip155 chain id parsed from the requested block header |  |

### Return type

[**crate::models::EthermintPeriodEvmPeriodV1PeriodQueryTraceTxResponse**](ethermint.evm.v1.QueryTraceTxResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unbonding_delegation

> crate::models::CosmosPeriodStakingPeriodV1beta1PeriodQueryUnbondingDelegationResponse unbonding_delegation(validator_addr, delegator_addr)
UnbondingDelegation queries unbonding info for given validator delegator pair.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**validator_addr** | **String** | validator_addr defines the validator address to query for. | [required] |
**delegator_addr** | **String** | delegator_addr defines the delegator address to query for. | [required] |

### Return type

[**crate::models::CosmosPeriodStakingPeriodV1beta1PeriodQueryUnbondingDelegationResponse**](cosmos.staking.v1beta1.QueryUnbondingDelegationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upgraded_consensus_state

> crate::models::CosmosPeriodUpgradePeriodV1beta1PeriodQueryUpgradedConsensusStateResponse upgraded_consensus_state(last_height)
UpgradedConsensusState queries the consensus state that will serve as a trusted kernel for the next version of this chain. It will only be stored at the last height of this chain. UpgradedConsensusState RPC not supported with legacy querier This rpc is deprecated now that IBC has its own replacement (https://github.com/cosmos/ibc-go/blob/2c880a22e9f9cc75f62b527ca94aa75ce1106001/proto/ibc/core/client/v1/query.proto#L54)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**last_height** | **String** | last height of the current chain must be sent in request as this is the height under which next consensus state is stored | [required] |

### Return type

[**crate::models::CosmosPeriodUpgradePeriodV1beta1PeriodQueryUpgradedConsensusStateResponse**](cosmos.upgrade.v1beta1.QueryUpgradedConsensusStateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validator

> crate::models::CosmosPeriodStakingPeriodV1beta1PeriodQueryValidatorResponse validator(validator_addr)
Validator queries validator info for given validator address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**validator_addr** | **String** | validator_addr defines the validator address to query for. | [required] |

### Return type

[**crate::models::CosmosPeriodStakingPeriodV1beta1PeriodQueryValidatorResponse**](cosmos.staking.v1beta1.QueryValidatorResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validator_account

> crate::models::EthermintPeriodEvmPeriodV1PeriodQueryValidatorAccountResponse validator_account(cons_address)
ValidatorAccount queries an Ethereum account's from a validator consensus Address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cons_address** | **String** | cons_address is the validator cons address to query the account for. | [required] |

### Return type

[**crate::models::EthermintPeriodEvmPeriodV1PeriodQueryValidatorAccountResponse**](ethermint.evm.v1.QueryValidatorAccountResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validator_commission

> crate::models::CosmosPeriodDistributionPeriodV1beta1PeriodQueryValidatorCommissionResponse validator_commission(validator_address)
ValidatorCommission queries accumulated commission for a validator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**validator_address** | **String** | validator_address defines the validator address to query for. | [required] |

### Return type

[**crate::models::CosmosPeriodDistributionPeriodV1beta1PeriodQueryValidatorCommissionResponse**](cosmos.distribution.v1beta1.QueryValidatorCommissionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validator_delegations

> crate::models::CosmosPeriodStakingPeriodV1beta1PeriodQueryValidatorDelegationsResponse validator_delegations(validator_addr, pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
ValidatorDelegations queries delegate info for given validator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**validator_addr** | **String** | validator_addr defines the validator address to query for. | [required] |
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodStakingPeriodV1beta1PeriodQueryValidatorDelegationsResponse**](cosmos.staking.v1beta1.QueryValidatorDelegationsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validator_distribution_info

> crate::models::CosmosPeriodDistributionPeriodV1beta1PeriodQueryValidatorDistributionInfoResponse validator_distribution_info(validator_address)
ValidatorDistributionInfo queries validator commision and self-delegation rewards for validator

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**validator_address** | **String** | validator_address defines the validator address to query for. | [required] |

### Return type

[**crate::models::CosmosPeriodDistributionPeriodV1beta1PeriodQueryValidatorDistributionInfoResponse**](cosmos.distribution.v1beta1.QueryValidatorDistributionInfoResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validator_outstanding_rewards

> crate::models::CosmosPeriodDistributionPeriodV1beta1PeriodQueryValidatorOutstandingRewardsResponse validator_outstanding_rewards(validator_address)
ValidatorOutstandingRewards queries rewards of a validator address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**validator_address** | **String** | validator_address defines the validator address to query for. | [required] |

### Return type

[**crate::models::CosmosPeriodDistributionPeriodV1beta1PeriodQueryValidatorOutstandingRewardsResponse**](cosmos.distribution.v1beta1.QueryValidatorOutstandingRewardsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validator_slashes

> crate::models::CosmosPeriodDistributionPeriodV1beta1PeriodQueryValidatorSlashesResponse validator_slashes(validator_address, starting_height, ending_height, pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
ValidatorSlashes queries slash events of a validator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**validator_address** | **String** | validator_address defines the validator address to query for. | [required] |
**starting_height** | Option<**String**> | starting_height defines the optional starting height to query the slashes. |  |
**ending_height** | Option<**String**> | starting_height defines the optional ending height to query the slashes. |  |
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodDistributionPeriodV1beta1PeriodQueryValidatorSlashesResponse**](cosmos.distribution.v1beta1.QueryValidatorSlashesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validator_unbonding_delegations

> crate::models::CosmosPeriodStakingPeriodV1beta1PeriodQueryValidatorUnbondingDelegationsResponse validator_unbonding_delegations(validator_addr, pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
ValidatorUnbondingDelegations queries unbonding delegations of a validator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**validator_addr** | **String** | validator_addr defines the validator address to query for. | [required] |
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodStakingPeriodV1beta1PeriodQueryValidatorUnbondingDelegationsResponse**](cosmos.staking.v1beta1.QueryValidatorUnbondingDelegationsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validators

> crate::models::CosmosPeriodStakingPeriodV1beta1PeriodQueryValidatorsResponse validators(status, pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
Validators queries all validators that match the given status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<**String**> | status enables to query for validators matching a given status. |  |
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodStakingPeriodV1beta1PeriodQueryValidatorsResponse**](cosmos.staking.v1beta1.QueryValidatorsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vote

> crate::models::CosmosPeriodGovPeriodV1PeriodQueryVoteResponse vote(proposal_id, voter)
Vote queries voted information based on proposalID, voterAddr.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_id** | **String** | proposal_id defines the unique id of the proposal. | [required] |
**voter** | **String** | voter defines the voter address for the proposals. | [required] |

### Return type

[**crate::models::CosmosPeriodGovPeriodV1PeriodQueryVoteResponse**](cosmos.gov.v1.QueryVoteResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vote_0

> crate::models::CosmosPeriodGovPeriodV1beta1PeriodQueryVoteResponse vote_0(proposal_id, voter)
Vote queries voted information based on proposalID, voterAddr.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_id** | **String** | proposal_id defines the unique id of the proposal. | [required] |
**voter** | **String** | voter defines the voter address for the proposals. | [required] |

### Return type

[**crate::models::CosmosPeriodGovPeriodV1beta1PeriodQueryVoteResponse**](cosmos.gov.v1beta1.QueryVoteResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vote_by_proposal_voter

> crate::models::CosmosPeriodGroupPeriodV1PeriodQueryVoteByProposalVoterResponse vote_by_proposal_voter(proposal_id, voter)
VoteByProposalVoter queries a vote by proposal id and voter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_id** | **String** | proposal_id is the unique ID of a proposal. | [required] |
**voter** | **String** | voter is a proposal voter account address. | [required] |

### Return type

[**crate::models::CosmosPeriodGroupPeriodV1PeriodQueryVoteByProposalVoterResponse**](cosmos.group.v1.QueryVoteByProposalVoterResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## votes

> crate::models::CosmosPeriodGovPeriodV1PeriodQueryVotesResponse votes(proposal_id, pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
Votes queries votes of a given proposal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_id** | **String** | proposal_id defines the unique id of the proposal. | [required] |
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodGovPeriodV1PeriodQueryVotesResponse**](cosmos.gov.v1.QueryVotesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## votes_0

> crate::models::CosmosPeriodGovPeriodV1beta1PeriodQueryVotesResponse votes_0(proposal_id, pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
Votes queries votes of a given proposal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_id** | **String** | proposal_id defines the unique id of the proposal. | [required] |
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodGovPeriodV1beta1PeriodQueryVotesResponse**](cosmos.gov.v1beta1.QueryVotesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## votes_by_proposal

> crate::models::CosmosPeriodGroupPeriodV1PeriodQueryVotesByProposalResponse votes_by_proposal(proposal_id, pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
VotesByProposal queries a vote by proposal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proposal_id** | **String** | proposal_id is the unique ID of a proposal. | [required] |
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodGroupPeriodV1PeriodQueryVotesByProposalResponse**](cosmos.group.v1.QueryVotesByProposalResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## votes_by_voter

> crate::models::CosmosPeriodGroupPeriodV1PeriodQueryVotesByVoterResponse votes_by_voter(voter, pagination_period_key, pagination_period_offset, pagination_period_limit, pagination_period_count_total, pagination_period_reverse)
VotesByVoter queries a vote by voter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**voter** | **String** | voter is a proposal voter account address. | [required] |
**pagination_period_key** | Option<**String**> | key is a value returned in PageResponse.next_key to begin querying the next page most efficiently. Only one of offset or key should be set. |  |
**pagination_period_offset** | Option<**String**> | offset is a numeric offset that can be used when key is unavailable. It is less efficient than using key. Only one of offset or key should be set. |  |
**pagination_period_limit** | Option<**String**> | limit is the total number of results to be returned in the result page. If left empty it will default to a value to be set by each app. |  |
**pagination_period_count_total** | Option<**bool**> | count_total is set to true  to indicate that the result set should include a count of the total number of items available for pagination in UIs. count_total is only respected when offset is used. It is ignored when key is set. |  |
**pagination_period_reverse** | Option<**bool**> | reverse is set to true if results are to be returned in the descending order.  Since: cosmos-sdk 0.43 |  |

### Return type

[**crate::models::CosmosPeriodGroupPeriodV1PeriodQueryVotesByVoterResponse**](cosmos.group.v1.QueryVotesByVoterResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

