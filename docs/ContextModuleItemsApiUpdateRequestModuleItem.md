# ContextModuleItemsApiUpdateRequestModuleItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | Option<**String**> | [String] The name of the module item | [optional]
**position** | Option<**i32**> | [Integer] The position of this item in the module (1-based) | [optional]
**indent** | Option<**i32**> | [Integer] 0-based indent level; module items may be indented to show a hierarchy | [optional]
**external_url** | Option<**String**> | [String] External url that the item points to. Only applies to 'ExternalUrl' type. | [optional]
**new_tab** | Option<**bool**> | [Boolean] Whether the external tool opens in a new tab. Only applies to 'ExternalTool' type. | [optional]
**completion_requirement** | **String** | [min_score] [Integer] Minimum score required to complete, Required for completion_requirement type 'min_score'. | 
**published** | Option<**bool**> | [Boolean] Whether the module item is published and visible to students. | [optional]
**module_id** | Option<**String**> | [String] Move this item to another module by specifying the target module id here. The target module must be in the same course. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


