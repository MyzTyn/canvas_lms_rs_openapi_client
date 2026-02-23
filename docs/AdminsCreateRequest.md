# AdminsCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | **i32** | The id of the user to promote. | 
**role** | Option<**String**> | [DEPRECATED] The user's admin relationship with the account will be created with the given role. Defaults to 'AccountAdmin'. | [optional]
**role_id** | Option<**i32**> | The user's admin relationship with the account will be created with the given role. Defaults to the built-in role for 'AccountAdmin'. | [optional]
**send_confirmation** | Option<**bool**> | Send a notification email to the new admin if true. Default is true. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


