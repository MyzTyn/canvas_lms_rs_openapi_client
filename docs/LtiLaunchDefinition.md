# LtiLaunchDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**definition_type** | Option<**String**> | The type of the launch definition. Always 'ContextExternalTool' | [optional]
**definition_id** | Option<**String**> | The Canvas ID of the tool | [optional]
**name** | Option<**String**> | The display name of the tool for the given placement | [optional]
**description** | Option<**String**> | The description of the tool for the given placement. | [optional]
**url** | Option<**String**> | The launch URL for the tool | [optional]
**domain** | Option<**String**> | The domain of the tool | [optional]
**placements** | Option<**serde_json::Value**> | Placement-specific config for given placements | [optional]
**context_name** | Option<**String**> | The name of the account or course where the tool is deployed. Only included if requested via include_context_name parameter. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


