# ServiceCredentialsCreateRequestPseudonym

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**unique_id** | **String** | [Required, String] User's login ID. If this is a self-registration, it must be a valid email address. | 
**password** | Option<**String**> | [String] User's password. Cannot be set during self-registration. | [optional]
**sis_user_id** | Option<**String**> | [String] SIS ID for the user's account. To set this parameter, the caller must be able to manage SIS permissions. | [optional]
**integration_id** | Option<**String**> | [String] Integration ID for the login. To set this parameter, the caller must be able to manage SIS permissions. The Integration ID is a secondary identifier useful for more complex SIS integrations. | [optional]
**send_confirmation** | Option<**bool**> | [Boolean] Send user notification of account creation if true. Automatically set to true during self-registration. | [optional]
**force_self_registration** | Option<**bool**> | [Boolean] Send user a self-registration style email if true. Setting it means the users will get a notification asking them to \"complete the registration process\" by clicking it, setting a password, and letting them in.  Will only be executed on if the user does not need admin approval. Defaults to false unless explicitly provided. | [optional]
**authentication_provider_id** | Option<**String**> | [String] The authentication provider this login is associated with. Logins associated with a specific provider can only be used with that provider. Legacy providers (LDAP, CAS, SAML) will search for logins associated with them, or unassociated logins. New providers will only search for logins explicitly associated with them. This can be the integer ID of the provider, or the type of the provider (in which case, it will find the first matching provider). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


