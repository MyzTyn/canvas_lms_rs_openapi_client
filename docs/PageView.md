# PageView

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | A UUID representing the page view.  This is also the unique request id | 
**app_name** | Option<**String**> | If the request is from an API request, the app that generated the access token | [optional]
**url** | Option<**String**> | The URL requested | [optional]
**context_type** | Option<**String**> | The type of context for the request | [optional]
**asset_type** | Option<**String**> | The type of asset in the context for the request, if any | [optional]
**controller** | Option<**String**> | The rails controller that handled the request | [optional]
**action** | Option<**String**> | The rails action that handled the request | [optional]
**contributed** | Option<**bool**> | This field is deprecated, and will always be false | [optional]
**interaction_seconds** | Option<**f64**> | An approximation of how long the user spent on the page, in seconds | [optional]
**created_at** | Option<**String**> | When the request was made | [optional]
**user_request** | Option<**bool**> | A flag indicating whether the request was user-initiated, or automatic (such as an AJAX call). Not available in history CSV. | [optional]
**render_time** | Option<**f64**> | How long the response took to render, in seconds. Not available in history CSV. | [optional]
**user_agent** | Option<**String**> | The user-agent of the browser or program that made the request | [optional]
**participated** | Option<**bool**> | True if the request counted as participating, such as submitting homework | [optional]
**http_method** | Option<**String**> | The HTTP method such as GET or POST | [optional]
**remote_ip** | Option<**String**> | The origin IP address of the request | [optional]
**session_id** | Option<**uuid::Uuid**> | The session identifier for the user session that made the request | [optional]
**developer_key_id** | Option<**f64**> | The ID of the developer key that authorized the API request, if applicable | [optional]
**asset_user_access_id** | Option<**f64**> | The ID of the asset (e.g. an assignment) associated with this page view, if applicable | [optional]
**links** | Option<[**models::PageViewLinks**](PageViewLinks.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


