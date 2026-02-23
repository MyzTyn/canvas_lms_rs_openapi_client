# LtiResourceLink

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The Canvas identifier for the LTI Resource Link. | [optional]
**context_id** | Option<**i32**> | The Canvas identifier for the context that the LTI Resource Link is associated with. | [optional]
**context_type** | Option<**String**> | The type of the context that the LTI Resource Link is associated with. | [optional]
**context_external_tool_id** | Option<**i32**> | The Canvas identifier for the LTI 1.3 External Tool that the LTI Resource Link was originally installed from. Note that this tool may have been deleted or reinstalled and may not be the tool that would be launched for this url. | [optional]
**resource_type** | Option<**String**> | The type of Canvas content for the resource link. Included for convenience. | [optional]
**canvas_launch_url** | Option<**String**> | The Canvas URL that launches the LTI Resource Link. Suitable for use in Canvas rich content | [optional]
**resource_link_uuid** | Option<**String**> | The LTI identifier for the LTI Resource Link, included as the resource_link_id when this link is launched | [optional]
**lookup_uuid** | Option<**String**> | A unique identifier for the LTI Resource Link, present in the rich content representation. Remains the same across content migration. | [optional]
**title** | Option<**String**> | The title of the LTI Resource Link. Usually tool-provided, or matches the assignment name | [optional]
**url** | Option<**String**> | The tool URL to which the LTI Resource Link will launch | [optional]
**lti_1_1_id** | Option<**String**> | The LTI 1.1 identifier for the LTI Resource Link, included in lti1p1 migration claim when launched. Only present if tool was migrated from 1.1 to 1.3. | [optional]
**created_at** | Option<**String**> | Timestamp of the resource link's creation | [optional]
**updated_at** | Option<**String**> | Timestamp of the resource link's last update | [optional]
**workflow_state** | Option<**String**> | The state of the resource link | [optional]
**associated_content_type** | Option<**String**> | Type of the associated content this resource link belongs to if present. Now only supports `ModuleItems`, later may be extend others | [optional]
**associated_content_id** | Option<**i32**> | The Canvas identifier of the associated content, e.g. ModuleItem related to this link. Present if associated_content_type is present | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


