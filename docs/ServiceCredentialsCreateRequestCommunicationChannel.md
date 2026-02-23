# ServiceCredentialsCreateRequestCommunicationChannel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | [String] The communication channel type, e.g. 'email' or 'sms'. | [optional]
**address** | Option<**String**> | [String] The communication channel address, e.g. the user's email address. | [optional]
**confirmation_url** | Option<**bool**> | [Boolean] Only valid for account admins. If true, returns the new user account confirmation URL in the response. | [optional]
**skip_confirmation** | Option<**bool**> | [Boolean] Only valid for site admins and account admins making requests; If true, the channel is automatically validated and no confirmation email or SMS is sent. Otherwise, the user must respond to a confirmation message to confirm the channel.  If this is true, it is recommended to set <tt>\"pseudonym[send_confirmation]\"</tt> to true as well. Otherwise, the user will not receive any messages about their account creation. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


