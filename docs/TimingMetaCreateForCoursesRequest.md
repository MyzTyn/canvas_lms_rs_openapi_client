# TimingMetaCreateForCoursesRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | **String** | The client id is attached to the developer key. If supplied all other parameters are unnecessary and will be ignored | 
**name** | **String** | The name of the tool | 
**privacy_level** | **PrivacyLevel** | How much user information to send to the external tool. (enum: anonymous, name_only, email_only, public) | 
**consumer_key** | **String** | The consumer key for the external tool | 
**shared_secret** | **String** | The shared secret with the external tool | 
**description** | Option<**String**> | A description of the tool | [optional]
**url** | Option<**String**> | The url to match links against. Either \"url\" or \"domain\" should be set, not both. | [optional]
**domain** | Option<**String**> | The domain to match links against. Either \"url\" or \"domain\" should be set, not both. | [optional]
**icon_url** | Option<**String**> | The url of the icon to show for this tool | [optional]
**text** | Option<**String**> | The default text to show for this tool | [optional]
**is_rce_favorite** | Option<**bool**> | (Deprecated in favor of {api:ExternalToolsController#mark_rce_favorite Mark tool to RCE Favorites} and {api:ExternalToolsController#unmark_rce_favorite Unmark tool from RCE Favorites}) Whether this tool should appear in a preferred location in the RCE. This only applies to tools in root account contexts that have an editor button placement. | [optional]
**left_square_bracket_less_than_placement_configuration_key_greater_than_right_square_bracket_left_square_bracket_placement_name_right_square_bracket** | Option<**String**> | [variable] Set the <placement_configuration_key> value for a specific placement. | [optional]
**config_type** | Option<**ConfigType**> | Configuration can be passed in as Common Cartridge XML instead of using query parameters. If this value is \"by_url\" or \"by_xml\" then an XML configuration will be expected in either the \"config_xml\" or \"config_url\" parameter. Note that the name parameter overrides the tool name provided in the XML. (enum: by_url, by_xml) | [optional]
**config_xml** | **String** | XML tool configuration, as specified in the Common Cartridge XML specification. This is required if \"config_type\" is set to \"by_xml\" | 
**config_url** | **String** | URL where the server can retrieve an XML tool configuration, as specified in the Common Cartridge XML specification. This is required if \"config_type\" is set to \"by_url\" | 
**not_selectable** | Option<**bool**> | Default: false. If set to true, and if resource_selection is set to false, the tool won't show up in the external tool selection UI in modules and assignments | [optional]
**oauth_compliant** | Option<**bool**> | Default: false, if set to true LTI query params will not be copied to the post body. | [optional]
**unified_tool_id** | Option<**String**> | The unique identifier for the tool in LearnPlatform | [optional]
**custom_fields** | Option<[**models::TimingMetaCreateForCoursesRequestCustomFields**](TimingMetaCreateForCoursesRequestCustomFields.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


