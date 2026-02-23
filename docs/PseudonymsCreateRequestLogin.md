# PseudonymsCreateRequestLogin

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**unique_id** | **String** | [Required, String] The unique ID for the new login. | 
**password** | Option<**String**> | [String] The new login's password. | [optional]
**sis_user_id** | Option<**String**> | [String] SIS ID for the login. To set this parameter, the caller must be able to manage SIS permissions on the account. | [optional]
**integration_id** | Option<**String**> | [String] Integration ID for the login. To set this parameter, the caller must be able to manage SIS permissions on the account. The Integration ID is a secondary identifier useful for more complex SIS integrations. | [optional]
**authentication_provider_id** | Option<**String**> | [String] The authentication provider this login is associated with. Logins associated with a specific provider can only be used with that provider. Legacy providers (LDAP, CAS, SAML) will search for logins associated with them, or unassociated logins. New providers will only search for logins explicitly associated with them. This can be the integer ID of the provider, or the type of the provider (in which case, it will find the first matching provider). | [optional]
**declared_user_type** | Option<**String**> | [String] The declared intention of the user type. This can be set, but does not change any Canvas functionality with respect to their access. A user can still be a teacher, admin, student, etc. in any particular context without regard to this setting. This can be used for administrative purposes for integrations to be able to more easily identify why the user was created. Valid values are:   * administrative   * observer   * staff   * student   * student_other   * teacher | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


