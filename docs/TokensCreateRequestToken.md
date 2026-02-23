# TokensCreateRequestToken

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**purpose** | **String** | [Required, String] The purpose of the token. | 
**expires_at** | Option<**String**> | [DateTime] The time at which the token will expire. | [optional]
**scopes** | Option<**String**> | [] [Array] The scopes to associate with the token. Ignored if the default developer key does not have the \"enable scopes\" option enabled. In such cases, the token will inherit the user's permissions instead. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


