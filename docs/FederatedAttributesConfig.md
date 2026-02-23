# FederatedAttributesConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**admin_roles** | Option<**String**> | A comma separated list of role names to grant to the user. Note that these only apply at the root account level, and not sub-accounts. If the attribute is not marked for provisioning only, the user will also be removed from any other roles they currently hold that are not still specified by the IdP. | [optional]
**display_name** | Option<**String**> | The full display name of the user | [optional]
**email** | Option<**String**> | The user's e-mail address | [optional]
**given_name** | Option<**String**> | The first, or given, name of the user | [optional]
**integration_id** | Option<**String**> | The secondary unique identifier for SIS purposes | [optional]
**locale** | Option<**String**> | The user's preferred locale/language | [optional]
**name** | Option<**String**> | The full name of the user | [optional]
**sis_user_id** | Option<**String**> | The unique SIS identifier | [optional]
**sortable_name** | Option<**String**> | The full name of the user for sorting purposes | [optional]
**surname** | Option<**String**> | The surname, or last name, of the user | [optional]
**timezone** | Option<**String**> | The user's preferred time zone | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


