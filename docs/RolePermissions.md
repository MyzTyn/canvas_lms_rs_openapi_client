# RolePermissions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled** | Option<**bool**> | Whether the role has the permission | [optional]
**locked** | Option<**bool**> | Whether the permission is locked by this role | [optional]
**applies_to_self** | Option<**bool**> | Whether the permission applies to the account this role is in. Only present if enabled is true | [optional]
**applies_to_descendants** | Option<**bool**> | Whether the permission cascades down to sub accounts of the account this role is in. Only present if enabled is true | [optional]
**readonly** | Option<**bool**> | Whether the permission can be modified in this role (i.e. whether the permission is locked by an upstream role). | [optional]
**explicit** | Option<**bool**> | Whether the value of enabled is specified explicitly by this role, or inherited from an upstream role. | [optional]
**prior_default** | Option<**bool**> | The value that would have been inherited from upstream if the role had not explicitly set a value. Only present if explicit is true. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


