# PseudonymsCreateRequestUser

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | [Required, String] The ID of the user to create the login for. | 
**existing_user_id** | Option<**String**> | [String] A Canvas User ID to identify a user in a trusted account (alternative to `id`, `existing_sis_user_id`, or `existing_integration_id`). This parameter is not available in OSS Canvas. | [optional]
**existing_integration_id** | Option<**String**> | [String] An Integration ID to identify a user in a trusted account (alternative to `id`, `existing_user_id`, or `existing_sis_user_id`). This parameter is not available in OSS Canvas. | [optional]
**existing_sis_user_id** | Option<**String**> | [String] An SIS User ID to identify a user in a trusted account (alternative to `id`, `existing_integration_id`, or `existing_user_id`). This parameter is not available in OSS Canvas. | [optional]
**trusted_account** | **String** | [String] The domain of the account to search for the user. This field is required when identifying a user in a trusted account. This parameter is not available in OSS Canvas. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


