# LineItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The fully qualified URL for showing, updating, and deleting the Line Item | [optional]
**score_maximum** | Option<**f64**> | The maximum score of the Line Item | [optional]
**label** | Option<**String**> | The label of the Line Item. | [optional]
**tag** | Option<**String**> | Tag used to qualify a line Item beyond its ids | [optional]
**resource_id** | Option<**String**> | A Tool Provider specified id for the Line Item. Multiple line items can share the same resourceId within a given context | [optional]
**resource_link_id** | Option<**String**> | The resource link id the Line Item is attached to | [optional]
**https_colon_slash_slash_canvas_instructure_com_slash_lti_slash_submission_type** | Option<**String**> | The extension that defines the submission_type of the line_item. Only returns if set through the line_item create endpoint. | [optional]
**https_colon_slash_slash_canvas_instructure_com_slash_lti_slash_launch_url** | Option<**String**> | The launch url of the Line Item. Only returned if `include=launch_url` query parameter is passed, and only for Show and List actions. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


