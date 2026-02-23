# LtiPlacement

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**placement** | Option<**String**> | The name of the placement. | [optional]
**enabled** | Option<**bool**> | If true, the tool will show in this placement. If false, it will not. | [optional]
**message_type** | Option<**String**> | Default message type for all placements | [optional]
**text** | Option<**String**> | The text of the link to the tool (if applicable). | [optional]
**labels** | Option<**serde_json::Value**> | Canvas-specific i18n for placement text. See the Navigation Placement docs. | [optional]
**custom_fields** | Option<**serde_json::Value**> | Placement-specific custom fields to send in the launch. Merged with tool-level custom fields. | [optional]
**selection_height** | Option<**f64**> | Default iframe height. Not valid for all placements. Overrides tool-level launch_height. | [optional]
**selection_width** | Option<**f64**> | Default iframe width. Not valid for all placements. Overrides tool-level launch_width. | [optional]
**launch_height** | Option<**f64**> | Default iframe height. Not valid for all placements. Overrides tool-level launch_height. | [optional]
**launch_width** | Option<**f64**> | Default iframe width. Not valid for all placements. Overrides tool-level launch_width. | [optional]
**icon_url** | Option<**String**> | Default icon URL. Not valid for all placements. Overrides tool-level icon_url. | [optional]
**canvas_icon_class** | Option<**String**> | The HTML class name of an InstUI Icon. Used instead of an icon_url in select placements. | [optional]
**required_permissions** | Option<**String**> | Comma-separated list of Canvas permission short names required for a user to launch from this placement. | [optional]
**window_target** | Option<**String**> | When set to '_blank', opens placement in a new tab. | [optional]
**display_type** | Option<**String**> | The Canvas layout to use when launching the tool. See the Navigation Placement docs. | [optional]
**url** | Option<**String**> | The 1.1 launch URL for this placement. Overrides tool-level url. | [optional]
**target_link_uri** | Option<**String**> | The 1.3 launch URL for this placement. Overrides tool-level target_link_uri. | [optional]
**visibility** | Option<**String**> | Specifies types of users that can see this placement. Only valid for some placements like course_navigation. | [optional]
**prefer_sis_email** | Option<**bool**> | 1.1 specific. If true, the tool will send the SIS email in the lis_person_contact_email_primary launch property | [optional]
**oauth_compliant** | Option<**bool**> | 1.1 specific. If true, query parameters from the launch URL will not be copied to the POST body. | [optional]
**icon_svg_path_64** | Option<**String**> | An SVG to use instead of an icon_url. Only valid for global_navigation. | [optional]
**default** | Option<**String**> | Default display state for course_navigation. If 'enabled', will show in course sidebar. If 'disabled', will be hidden. | [optional]
**accept_media_types** | Option<**String**> | Comma-separated list of media types that the tool can accept. Only valid for file_item. | [optional]
**use_tray** | Option<**bool**> | If true, the tool will be launched in the tray. Only used by the editor_button placement. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


