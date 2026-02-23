# CommunicationChannelsCreateRequestCommunicationChannel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | **String** | [Required, String] An email address or SMS number. Not required for \"push\" type channels. | 
**r#type** | **Type** | [Required, String, \"email\"|\"sms\"|\"push\"] The type of communication channel.  In order to enable push notification support, the server must be properly configured (via `sns_creds` in Vault) to communicate with Amazon Simple Notification Services, and the developer key used to create the access token from this request must have an SNS ARN configured on it. (enum: email, sms, push) | 
**token** | Option<**String**> | [String] A registration id, device token, or equivalent token given to an app when registering with a push notification provider. Only valid for \"push\" type channels. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


