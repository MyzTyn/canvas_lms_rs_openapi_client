# BadPermissionSettingErrorAddRoleRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**label** | **String** | Label for the role. | 
**role** | Option<**String**> | Deprecated alias for label. | [optional]
**base_role_type** | Option<**BaseRoleType**> | Specifies the role type that will be used as a base for the permissions granted to this role.  Defaults to 'AccountMembership' if absent (enum: AccountMembership, StudentEnrollment, TeacherEnrollment, TaEnrollment, ObserverEnrollment, DesignerEnrollment) | [optional]
**permissions** | Option<[**models::BadPermissionSettingErrorAddRoleRequestPermissions**](BadPermissionSettingErrorAddRoleRequestPermissions.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


