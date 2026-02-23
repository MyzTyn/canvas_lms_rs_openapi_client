# ContextModuleItemsApiCreateRequestModuleItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | Option<**String**> | [String] The name of the module item and associated content | [optional]
**r#type** | **Type** | [Required, String, \"File\"|\"Page\"|\"Discussion\"|\"Assignment\"|\"Quiz\"|\"SubHeader\"|\"ExternalUrl\"|\"ExternalTool\"] The type of content linked to the item (enum: File, Page, Discussion, Assignment, Quiz, SubHeader, ExternalUrl, ExternalTool) | 
**content_id** | **String** | [Required, String] The id of the content to link to the module item. Required, except for 'ExternalUrl', 'Page', and 'SubHeader' types. | 
**position** | Option<**i32**> | [Integer] The position of this item in the module (1-based). | [optional]
**indent** | Option<**i32**> | [Integer] 0-based indent level; module items may be indented to show a hierarchy | [optional]
**page_url** | **String** | [String] Suffix for the linked wiki page (e.g. 'front-page'). Required for 'Page' type. | 
**external_url** | **String** | [String] External url that the item points to. [Required for 'ExternalUrl' and 'ExternalTool' types. | 
**new_tab** | Option<**bool**> | [Boolean] Whether the external tool opens in a new tab. Only applies to 'ExternalTool' type. | [optional]
**completion_requirement** | **String** | [min_score] [Integer] Minimum score required to complete. Required for completion_requirement type 'min_score'. | 
**iframe** | Option<**String**> | [height] [Integer] Height of the ExternalTool on launch | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


