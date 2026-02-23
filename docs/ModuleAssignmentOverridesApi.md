# \ModuleAssignmentOverridesApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**module_assignment_overrides_bulk_update**](ModuleAssignmentOverridesApi.md#module_assignment_overrides_bulk_update) | **PUT** /courses/{course_id}/modules/{context_module_id}/assignment_overrides | Update a module's overrides
[**module_assignment_overrides_index**](ModuleAssignmentOverridesApi.md#module_assignment_overrides_index) | **GET** /courses/{course_id}/modules/{context_module_id}/assignment_overrides | List a module's overrides



## module_assignment_overrides_bulk_update

> module_assignment_overrides_bulk_update(context_module_id, course_id, module_assignment_overrides_bulk_update_request)
Update a module's overrides

Accepts a list of overrides and applies them to the ContextModule. Returns 204 No Content response code if successful.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_module_id** | **String** | Scope response to context_module_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**module_assignment_overrides_bulk_update_request** | Option<[**ModuleAssignmentOverridesBulkUpdateRequest**](ModuleAssignmentOverridesBulkUpdateRequest.md)> | Request body parameters  **Example Request:** ``` curl https://<canvas>/api/v1/courses/:course_id/modules/:context_module_id/assignment_overrides \\   -X PUT \\   -H 'Authorization: Bearer <token>' \\   -H 'Content-Type: application/json' \\   -d '{         \"overrides\": [           {             \"id\": 212,             \"course_section_id\": 3564           },           {             \"id\": 56,             \"group_id\": 7809           },           {             \"title\": \"an assignment override\",             \"student_ids\": [1, 2, 3]           }         ]       }' ``` |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## module_assignment_overrides_index

> models::ModuleAssignmentOverride module_assignment_overrides_index(context_module_id, course_id, page, per_page)
List a module's overrides

Returns a paginated list of AssignmentOverrides that apply to the ContextModule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_module_id** | **String** | Scope response to context_module_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::ModuleAssignmentOverride**](ModuleAssignmentOverride.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

