# \BlockEditorTemplatesApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**block_editor_templates_api_index**](BlockEditorTemplatesApiApi.md#block_editor_templates_api_index) | **GET** /courses/{course_id}/block_editor_templates | List block templates



## block_editor_templates_api_index

> models::BlockEditorTemplate block_editor_templates_api_index(course_id, sort, order, drafts, r#type, include)
List block templates

A list of the block templates available to the current user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**sort** | Option<**String**> | Sort results by this field. |  |
**order** | Option<**String**> | The sorting order. Defaults to 'asc'. |  |
**drafts** | Option<**bool**> | If true, include draft templates. If false or omitted only published templates will be returned. |  |
**r#type** | Option<[**Vec<String>**](String.md)> | [String, \"page\"|\"section\"|\"block\"] What type of templates should be returned. |  |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"node_tree\" | \"thumbnail\"] |  |

### Return type

[**models::BlockEditorTemplate**](BlockEditorTemplate.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

