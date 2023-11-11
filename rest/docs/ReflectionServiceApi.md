# \ReflectionServiceApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_authn_descriptor**](ReflectionServiceApi.md#get_authn_descriptor) | **GET** /cosmos/base/reflection/v1beta1/app_descriptor/authn | GetAuthnDescriptor returns information on how to authenticate transactions in the application NOTE: this RPC is still experimental and might be subject to breaking changes or removal in future releases of the cosmos-sdk.
[**get_chain_descriptor**](ReflectionServiceApi.md#get_chain_descriptor) | **GET** /cosmos/base/reflection/v1beta1/app_descriptor/chain | GetChainDescriptor returns the description of the chain
[**get_codec_descriptor**](ReflectionServiceApi.md#get_codec_descriptor) | **GET** /cosmos/base/reflection/v1beta1/app_descriptor/codec | GetCodecDescriptor returns the descriptor of the codec of the application
[**get_configuration_descriptor**](ReflectionServiceApi.md#get_configuration_descriptor) | **GET** /cosmos/base/reflection/v1beta1/app_descriptor/configuration | GetConfigurationDescriptor returns the descriptor for the sdk.Config of the application
[**get_query_services_descriptor**](ReflectionServiceApi.md#get_query_services_descriptor) | **GET** /cosmos/base/reflection/v1beta1/app_descriptor/query_services | GetQueryServicesDescriptor returns the available gRPC queryable services of the application
[**get_tx_descriptor**](ReflectionServiceApi.md#get_tx_descriptor) | **GET** /cosmos/base/reflection/v1beta1/app_descriptor/tx_descriptor | GetTxDescriptor returns information on the used transaction object and available msgs that can be used
[**list_all_interfaces**](ReflectionServiceApi.md#list_all_interfaces) | **GET** /cosmos/base/reflection/v1beta1/interfaces | ListAllInterfaces lists all the interfaces registered in the interface registry.
[**list_implementations**](ReflectionServiceApi.md#list_implementations) | **GET** /cosmos/base/reflection/v1beta1/interfaces/{interfaceName}/implementations | ListImplementations list all the concrete types that implement a given interface.



## get_authn_descriptor

> crate::models::CosmosPeriodBasePeriodReflectionPeriodV2alpha1PeriodGetAuthnDescriptorResponse get_authn_descriptor()
GetAuthnDescriptor returns information on how to authenticate transactions in the application NOTE: this RPC is still experimental and might be subject to breaking changes or removal in future releases of the cosmos-sdk.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CosmosPeriodBasePeriodReflectionPeriodV2alpha1PeriodGetAuthnDescriptorResponse**](cosmos.base.reflection.v2alpha1.GetAuthnDescriptorResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chain_descriptor

> crate::models::CosmosPeriodBasePeriodReflectionPeriodV2alpha1PeriodGetChainDescriptorResponse get_chain_descriptor()
GetChainDescriptor returns the description of the chain

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CosmosPeriodBasePeriodReflectionPeriodV2alpha1PeriodGetChainDescriptorResponse**](cosmos.base.reflection.v2alpha1.GetChainDescriptorResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_codec_descriptor

> crate::models::CosmosPeriodBasePeriodReflectionPeriodV2alpha1PeriodGetCodecDescriptorResponse get_codec_descriptor()
GetCodecDescriptor returns the descriptor of the codec of the application

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CosmosPeriodBasePeriodReflectionPeriodV2alpha1PeriodGetCodecDescriptorResponse**](cosmos.base.reflection.v2alpha1.GetCodecDescriptorResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_configuration_descriptor

> crate::models::CosmosPeriodBasePeriodReflectionPeriodV2alpha1PeriodGetConfigurationDescriptorResponse get_configuration_descriptor()
GetConfigurationDescriptor returns the descriptor for the sdk.Config of the application

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CosmosPeriodBasePeriodReflectionPeriodV2alpha1PeriodGetConfigurationDescriptorResponse**](cosmos.base.reflection.v2alpha1.GetConfigurationDescriptorResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_query_services_descriptor

> crate::models::CosmosPeriodBasePeriodReflectionPeriodV2alpha1PeriodGetQueryServicesDescriptorResponse get_query_services_descriptor()
GetQueryServicesDescriptor returns the available gRPC queryable services of the application

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CosmosPeriodBasePeriodReflectionPeriodV2alpha1PeriodGetQueryServicesDescriptorResponse**](cosmos.base.reflection.v2alpha1.GetQueryServicesDescriptorResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tx_descriptor

> crate::models::CosmosPeriodBasePeriodReflectionPeriodV2alpha1PeriodGetTxDescriptorResponse get_tx_descriptor()
GetTxDescriptor returns information on the used transaction object and available msgs that can be used

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CosmosPeriodBasePeriodReflectionPeriodV2alpha1PeriodGetTxDescriptorResponse**](cosmos.base.reflection.v2alpha1.GetTxDescriptorResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_all_interfaces

> crate::models::CosmosPeriodBasePeriodReflectionPeriodV1beta1PeriodListAllInterfacesResponse list_all_interfaces()
ListAllInterfaces lists all the interfaces registered in the interface registry.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CosmosPeriodBasePeriodReflectionPeriodV1beta1PeriodListAllInterfacesResponse**](cosmos.base.reflection.v1beta1.ListAllInterfacesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_implementations

> crate::models::CosmosPeriodBasePeriodReflectionPeriodV1beta1PeriodListImplementationsResponse list_implementations(interface_name)
ListImplementations list all the concrete types that implement a given interface.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**interface_name** | **String** | interface_name defines the interface to query the implementations for. | [required] |

### Return type

[**crate::models::CosmosPeriodBasePeriodReflectionPeriodV1beta1PeriodListImplementationsResponse**](cosmos.base.reflection.v1beta1.ListImplementationsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

