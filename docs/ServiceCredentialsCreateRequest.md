# ServiceCredentialsCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**force_validations** | Option<**bool**> | If true, validations are performed on the newly created user (and their associated pseudonym) even if the request is made by a privileged user like an admin. When set to false, or not included in the request parameters, any newly created users are subject to validations unless the request is made by a user with a 'manage_user_logins' right. In which case, certain validations such as 'require_acceptance_of_terms' and 'require_presence_of_name' are not enforced. Use this parameter to return helpful json errors while building users with an admin request. | [optional]
**enable_sis_reactivation** | Option<**bool**> | When true, will first try to re-activate a deleted user with matching sis_user_id if possible. This is commonly done with +user[skip_registration]+ and +communication_channel[skip_confirmation]+ so that the default communication_channel is also restored. | [optional]
**initial_enrollment_type** | Option<**String**> | `observer` if doing a self-registration with a pairing code. This allows setting the password during user creation. | [optional]
**user** | [**models::ServiceCredentialsCreateRequestUser**](ServiceCredentialsCreateRequestUser.md) |  | 
**pseudonym** | [**models::ServiceCredentialsCreateRequestPseudonym**](ServiceCredentialsCreateRequestPseudonym.md) |  | 
**communication_channel** | Option<[**models::ServiceCredentialsCreateRequestCommunicationChannel**](ServiceCredentialsCreateRequestCommunicationChannel.md)> |  | [optional]
**destination** | Option<[**models::ServiceCredentialsCreateRequestDestination**](ServiceCredentialsCreateRequestDestination.md)> |  | [optional]
**pairing_code** | Option<[**models::ServiceCredentialsCreateRequestPairingCode**](ServiceCredentialsCreateRequestPairingCode.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


