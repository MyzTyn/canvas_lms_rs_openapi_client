# JwtsCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**workflows** | Option<**Vec<String>**> | [String] Adds additional data to the JWT to be used by the consuming service workflow | [optional]
**context_type** | Option<**ContextType**> | The type of the context to generate the JWT for, in case the workflow requires it. Case insensitive. (enum: Course, User, Account) | [optional]
**context_id** | Option<**i32**> | The id of the context to generate the JWT for, in case the workflow requires it. | [optional]
**context_uuid** | Option<**String**> | The uuid of the context to generate the JWT for, in case the workflow requires it. Note that context_id and context_uuid are mutually exclusive. If both are provided, an error will be returned. | [optional]
**canvas_audience** | Option<**bool**> | Defaults to true. If false, the JWT will be signed, but not encrypted, for use in downstream services. The default encrypted behaviour can be used to talk to Canvas itself. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


