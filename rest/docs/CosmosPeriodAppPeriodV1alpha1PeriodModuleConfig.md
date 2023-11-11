# CosmosPeriodAppPeriodV1alpha1PeriodModuleConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | name is the unique name of the module within the app. It should be a name that persists between different versions of a module so that modules can be smoothly upgraded to new versions.  For example, for the module cosmos.bank.module.v1.Module, we may chose to simply name the module \"bank\" in the app. When we upgrade to cosmos.bank.module.v2.Module, the app-specific name \"bank\" stays the same and the framework knows that the v2 module should receive all the same state that the v1 module had. Note: modules should provide info on which versions they can migrate from in the ModuleDescriptor.can_migration_from field. | [optional]
**config** | Option<[**crate::models::GooglePeriodProtobufPeriodAny**](google.protobuf.Any.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


