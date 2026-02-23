# AuthorizationJwt

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sub** | Option<**String**> | The Tool Proxy Guid OR Developer key ID. A developer key ID should only be used if a tool proxy has not been created in Canvas. In this case the token should be signed with the developer key rather than the tool proxy shared secret. | [optional]
**aud** | Option<**String**> | The LTI 2 token authorization endpoint, can be found in the Tool Consumer Profile | [optional]
**exp** | Option<**i32**> | When this token expires, should be no more than 1 minute in the future | [optional]
**iat** | Option<**i32**> | The time this token was created | [optional]
**jti** | Option<**String**> | A unique ID for this token. Should be a UUID | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


