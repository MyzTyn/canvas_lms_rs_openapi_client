# \AssignmentGroupsApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assignment_groups_api_create**](AssignmentGroupsApiApi.md#assignment_groups_api_create) | **POST** /courses/{course_id}/assignment_groups | Create an Assignment Group
[**assignment_groups_api_destroy**](AssignmentGroupsApiApi.md#assignment_groups_api_destroy) | **DELETE** /courses/{course_id}/assignment_groups/{assignment_group_id} | Destroy an Assignment Group
[**assignment_groups_api_show**](AssignmentGroupsApiApi.md#assignment_groups_api_show) | **GET** /courses/{course_id}/assignment_groups/{assignment_group_id} | Get an Assignment Group
[**assignment_groups_api_update**](AssignmentGroupsApiApi.md#assignment_groups_api_update) | **PUT** /courses/{course_id}/assignment_groups/{assignment_group_id} | Edit an Assignment Group



## assignment_groups_api_create

> String assignment_groups_api_create(course_id, assignment_groups_api_create_request)
Create an Assignment Group

Create a new assignment group for this course.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**assignment_groups_api_create_request** | Option<[**AssignmentGroupsApiCreateRequest**](AssignmentGroupsApiCreateRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assignment_groups_api_destroy

> String assignment_groups_api_destroy(assignment_group_id, course_id, move_assignments_to)
Destroy an Assignment Group

Deletes the assignment group with the given id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_group_id** | **String** | Scope response to assignment_group_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**move_assignments_to** | Option<**i32**> | The ID of an active Assignment Group to which the assignments that are currently assigned to the destroyed Assignment Group will be assigned. NOTE: If this argument is not provided, any assignments in this Assignment Group will be deleted. |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assignment_groups_api_show

> String assignment_groups_api_show(assignment_group_id, course_id, include, override_assignment_dates, grading_period_id)
Get an Assignment Group

Returns the assignment group with the given id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_group_id** | **String** | Scope response to assignment_group_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [\"assignments\"|\"discussion_topic\"|\"assignment_visibility\"|\"submission\"|\"score_statistics\"] Associations to include with the group. \"discussion_topic\" and \"assignment_visibility\" and \"submission\" are only valid if \"assignments\" is also included. \"score_statistics\" is only valid if \"submission\" and \"assignments\" are also included. The \"assignment_visibility\" option additionally requires that the Differentiated Assignments course feature be turned on. |  |
**override_assignment_dates** | Option<**bool**> | Apply assignment overrides for each assignment, defaults to true. |  |
**grading_period_id** | Option<**i32**> | The id of the grading period in which assignment groups are being requested (Requires grading periods to exist on the account) |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assignment_groups_api_update

> String assignment_groups_api_update(assignment_group_id, course_id, assignment_groups_api_update_request)
Edit an Assignment Group

Modify an existing Assignment Group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_group_id** | **String** | Scope response to assignment_group_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**assignment_groups_api_update_request** | Option<[**AssignmentGroupsApiUpdateRequest**](AssignmentGroupsApiUpdateRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

