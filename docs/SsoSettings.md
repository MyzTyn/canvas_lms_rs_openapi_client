# SsoSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**login_handle_name** | Option<**String**> | The label used for unique login identifiers. | [optional]
**change_password_url** | Option<**String**> | The url to redirect users to for password resets. Leave blank for default Canvas behavior | [optional]
**auth_discovery_url** | Option<**String**> | If a discovery url is set, canvas will forward all users to that URL when they need to be authenticated. That page will need to then help the user figure out where they need to go to log in. If no discovery url is configured, the first configuration will be used to attempt to authenticate the user. | [optional]
**unknown_user_url** | Option<**String**> | If an unknown user url is set, Canvas will forward to that url when a service authenticates a user, but that user does not exist in Canvas. The default behavior is to present an error. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


