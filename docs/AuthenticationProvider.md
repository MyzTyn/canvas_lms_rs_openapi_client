# AuthenticationProvider

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**identifier_format** | Option<**String**> | Valid for SAML providers. | [optional]
**auth_type** | Option<**String**> | Valid for all providers. | [optional]
**id** | Option<**i32**> | Valid for all providers. | [optional]
**log_out_url** | Option<**String**> | Valid for SAML providers. | [optional]
**log_in_url** | Option<**String**> | Valid for SAML and CAS providers. | [optional]
**certificate_fingerprint** | Option<**String**> | Valid for SAML providers. | [optional]
**requested_authn_context** | Option<**String**> | Valid for SAML providers. | [optional]
**auth_host** | Option<**String**> | Valid for LDAP providers. | [optional]
**auth_filter** | Option<**String**> | Valid for LDAP providers. | [optional]
**auth_over_tls** | Option<**i32**> | Valid for LDAP providers. | [optional]
**auth_base** | Option<**String**> | Valid for LDAP and CAS providers. | [optional]
**auth_username** | Option<**String**> | Valid for LDAP providers. | [optional]
**auth_port** | Option<**i32**> | Valid for LDAP providers. | [optional]
**position** | Option<**i32**> | Valid for all providers. | [optional]
**idp_entity_id** | Option<**String**> | Valid for SAML providers. | [optional]
**login_attribute** | Option<**String**> | Valid for SAML providers. | [optional]
**sig_alg** | Option<**String**> | Valid for SAML providers. | [optional]
**jit_provisioning** | Option<**bool**> | Just In Time provisioning. Valid for all providers except Canvas (which has the similar in concept self_registration setting). | [optional]
**federated_attributes** | Option<[**models::FederatedAttributesConfig**](FederatedAttributesConfig.md)> |  | [optional]
**mfa_required** | Option<**bool**> | If multi-factor authentication is required when logging in with this authentication provider. The account must not have MFA disabled. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


