# LtiOverlay

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | Option<**String**> | The display name of the tool | [optional]
**description** | Option<**String**> | The description of the tool | [optional]
**custom_fields** | Option<**serde_json::Value**> | A key-value listing of all custom fields the tool has requested | [optional]
**target_link_uri** | Option<**String**> | The default launch URL for the tool. Overridable by placements. | [optional]
**domain** | Option<**String**> | The tool's main domain. Highly recommended for deep linking, used to match links to the tool. | [optional]
**privacy_level** | Option<**String**> | Canvas-defined privacy level for the tool | [optional]
**oidc_initiation_url** | Option<**String**> | 1.3 specific. URL used for initial login request | [optional]
**disabled_scopes** | Option<**Vec<String>**> | 1.3 specific. List of LTI scopes that the tool has requested but an admin has disabled | [optional]
**disabled_placements** | Option<**Vec<String>**> | List of placements that the tool has requested but an admin has disabled | [optional]
**placements** | Option<**serde_json::Value**> | Placement-specific settings changed by an admin | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


