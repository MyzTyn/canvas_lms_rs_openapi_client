# ContextExternalToolPlacement

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled** | Option<**bool**> | Whether this placement is enabled | [optional]
**url** | Option<**String**> | The launch URL for this specific placement. Overrides the tool's default URL. For LTI 1.1 tools only. | [optional]
**target_link_uri** | Option<**String**> | The launch URL for this specific placement. Overrides the tool's default target_link_uri. For LTI 1.3 tools only. | [optional]
**text** | Option<**String**> | The text/label to display for this placement. Overridable by 'labels' in placement configuration. | [optional]
**label** | Option<**String**> | The localized label for this placement. This is the resolved text after applying internationalization. | [optional]
**labels** | Option<**serde_json::Value**> | Internationalization labels for this placement. Keys are locale codes, values are localized text. | [optional]
**message_type** | Option<**String**> | The LTI message type for this placement. Not all placements support all message types. | [optional]
**selection_width** | Option<**i32**> | The width of the iframe or popup for this placement | [optional]
**selection_height** | Option<**i32**> | The height of the iframe or popup for this placement | [optional]
**launch_width** | Option<**i32**> | The width of the launch window. Not standard everywhere yet. | [optional]
**launch_height** | Option<**i32**> | The height of the launch window. Not standard everywhere yet. | [optional]
**icon_url** | Option<**String**> | The URL of the icon for this placement | [optional]
**canvas_icon_class** | Option<**String**> | The Canvas icon class to use for this placement instead of an icon URL | [optional]
**allow_fullscreen** | Option<**bool**> | Whether to allow fullscreen mode for this placement (top_navigation placement only) | [optional]
**custom_fields** | Option<**serde_json::Value**> | Custom fields to be sent with this placement's launch. Merged with tool-level custom fields. | [optional]
**visibility** | Option<**String**> | Controls who can see this placement | [optional]
**required_permissions** | Option<**String**> | Comma-separated list of Canvas permissions required to launch from this placement. The user must have all permissions in order to launch the tool. | [optional]
**default** | Option<**String**> | Default display state for navigation placements. Only applies to account_navigation and course_navigation placements. | [optional]
**display_type** | Option<**String**> | The layout type to use when launching the tool. For global_navigation and analytics_hub, defaults to 'full_width'. | [optional]
**window_target** | Option<**String**> | When set to '_blank', opens placement in a new tab. Only '_blank' is supported. | [optional]
**accept_media_types** | Option<**String**> | Comma-separated list of media types that the tool can accept. Only valid for file_menu placement. | [optional]
**use_tray** | Option<**bool**> | If true, the tool will be launched in the tray. Only used by the editor_button placement. | [optional]
**icon_svg_path_64** | Option<**String**> | An SVG path to use instead of an icon_url. Only valid for global_navigation placement. | [optional]
**root_account_only** | Option<**bool**> | Whether this placement should only be available at the root account level. Only applies to account_navigation placement. | [optional]
**description** | Option<**String**> | A description of this placement. Only valid for submission_type_selection placement. Maximum length of 255 characters. | [optional]
**require_resource_selection** | Option<**bool**> | Whether resource selection is required for this placement. Only valid for submission_type_selection placement. | [optional]
**prefer_sis_email** | Option<**bool**> | If true, the tool will send the SIS email in the lis_person_contact_email_primary launch property. LTI 1.1 only. | [optional]
**oauth_compliant** | Option<**bool**> | If true, query parameters from the launch URL will not be copied to the POST body. LTI 1.1 only. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


