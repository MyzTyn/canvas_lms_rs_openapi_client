# \AssignmentOverridesApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assignment_overrides_batch_create**](AssignmentOverridesApi.md#assignment_overrides_batch_create) | **POST** /courses/{course_id}/assignments/overrides | Batch create overrides in a course
[**assignment_overrides_batch_retrieve**](AssignmentOverridesApi.md#assignment_overrides_batch_retrieve) | **GET** /courses/{course_id}/assignments/overrides | Batch retrieve overrides in a course
[**assignment_overrides_batch_update**](AssignmentOverridesApi.md#assignment_overrides_batch_update) | **PUT** /courses/{course_id}/assignments/overrides | Batch update overrides in a course
[**assignment_overrides_create**](AssignmentOverridesApi.md#assignment_overrides_create) | **POST** /courses/{course_id}/assignments/{assignment_id}/overrides | Create an assignment override
[**assignment_overrides_destroy**](AssignmentOverridesApi.md#assignment_overrides_destroy) | **DELETE** /courses/{course_id}/assignments/{assignment_id}/overrides/{id} | Delete an assignment override
[**assignment_overrides_group_alias**](AssignmentOverridesApi.md#assignment_overrides_group_alias) | **GET** /groups/{group_id}/assignments/{assignment_id}/override | Redirect to the assignment override for a group
[**assignment_overrides_index**](AssignmentOverridesApi.md#assignment_overrides_index) | **GET** /courses/{course_id}/assignments/{assignment_id}/overrides | List assignment overrides
[**assignment_overrides_section_alias**](AssignmentOverridesApi.md#assignment_overrides_section_alias) | **GET** /sections/{course_section_id}/assignments/{assignment_id}/override | Redirect to the assignment override for a section
[**assignment_overrides_show**](AssignmentOverridesApi.md#assignment_overrides_show) | **GET** /courses/{course_id}/assignments/{assignment_id}/overrides/{id} | Get a single assignment override
[**assignment_overrides_update**](AssignmentOverridesApi.md#assignment_overrides_update) | **PUT** /courses/{course_id}/assignments/{assignment_id}/overrides/{id} | Update an assignment override



## assignment_overrides_batch_create

> models::AssignmentOverride assignment_overrides_batch_create(course_id, assignment_overrides_batch_create_request)
Batch create overrides in a course

Creates the specified overrides for each assignment.  Handles creation in a transaction, so all records are created or none are.  One of student_ids, group_id, or course_section_id must be present. At most one should be present; if multiple are present only the most specific (student_ids first, then group_id, then course_section_id) is used and any others are ignored.  Errors are reported in an errors attribute, an array of errors corresponding to inputs.  Global errors will be reported as a single element errors array

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**assignment_overrides_batch_create_request** | Option<[**AssignmentOverridesBatchCreateRequest**](AssignmentOverridesBatchCreateRequest.md)> | Request body parameters  **Example Request:** ``` curl \"https://<canvas>/api/v1/courses/12/assignments/overrides.json\" \\      -X POST \\      -F \"assignment_overrides[][assignment_id]=109\" \\      -F 'assignment_overrides[][student_ids][]=8' \\      -F \"assignment_overrides[][title]=foo\" \\      -F \"assignment_overrides[][assignment_id]=13\" \\      -F \"assignment_overrides[][course_section_id]=200\" \\      -F \"assignment_overrides[][due_at]=2012-10-08T21:00:00Z\" \\      -H \"Authorization: Bearer <token>\" ``` |  |

### Return type

[**models::AssignmentOverride**](AssignmentOverride.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assignment_overrides_batch_retrieve

> models::AssignmentOverride assignment_overrides_batch_retrieve(course_id, assignment_overrides_left_square_bracket_right_square_bracket_left_square_bracket_id_right_square_bracket, assignment_overrides_left_square_bracket_right_square_bracket_left_square_bracket_assignment_id_right_square_bracket)
Batch retrieve overrides in a course

Returns a list of specified overrides in this course, providing they target sections/groups/students visible to the current user. Returns null elements in the list for requests that were not found.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**assignment_overrides_left_square_bracket_right_square_bracket_left_square_bracket_id_right_square_bracket** | **String** | [Required, String] Ids of overrides to retrieve | [required] |
**assignment_overrides_left_square_bracket_right_square_bracket_left_square_bracket_assignment_id_right_square_bracket** | **String** | [Required, String] Ids of assignments for each override | [required] |

### Return type

[**models::AssignmentOverride**](AssignmentOverride.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assignment_overrides_batch_update

> models::AssignmentOverride assignment_overrides_batch_update(course_id, assignment_overrides_batch_update_request)
Batch update overrides in a course

Updates a list of specified overrides for each assignment.  Handles overrides in a transaction, so either all updates are applied or none. See {api:AssignmentOverridesController#update Update an assignment override} for available attributes.  All current overridden values must be supplied if they are to be retained; e.g. if due_at was overridden, but this PUT omits a value for due_at, due_at will no longer be overridden. If the override is adhoc and student_ids is not supplied, the target override set is unchanged. Target override sets cannot be changed for group or section overrides.  Errors are reported in an errors attribute, an array of errors corresponding to inputs.  Global errors will be reported as a single element errors array

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**assignment_overrides_batch_update_request** | Option<[**AssignmentOverridesBatchUpdateRequest**](AssignmentOverridesBatchUpdateRequest.md)> | Request body parameters  **Example Request:** ``` curl \"https://<canvas>/api/v1/courses/12/assignments/overrides.json\" \\      -X PUT \\      -F \"assignment_overrides[][id]=122\" \\      -F \"assignment_overrides[][assignment_id]=109\" \\      -F \"assignment_overrides[][title]=foo\" \\      -F \"assignment_overrides[][id]=993\" \\      -F \"assignment_overrides[][assignment_id]=13\" \\      -F \"assignment_overrides[][due_at]=2012-10-08T21:00:00Z\" \\      -H \"Authorization: Bearer <token>\" ``` |  |

### Return type

[**models::AssignmentOverride**](AssignmentOverride.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assignment_overrides_create

> String assignment_overrides_create(assignment_id, course_id, assignment_overrides_create_request)
Create an assignment override

One of student_ids, group_id, or course_section_id must be present. At most one should be present; if multiple are present only the most specific (student_ids first, then group_id, then course_section_id) is used and any others are ignored.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**assignment_overrides_create_request** | Option<[**AssignmentOverridesCreateRequest**](AssignmentOverridesCreateRequest.md)> | Request body parameters  **Example Request:** ``` curl 'https://<canvas>/api/v1/courses/1/assignments/2/overrides.json' \\      -X POST \\      -F 'assignment_override[student_ids][]=8' \\      -F 'assignment_override[title]=Fred Flinstone' \\      -F 'assignment_override[due_at]=2012-10-08T21:00:00Z' \\      -H \"Authorization: Bearer <token>\" ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assignment_overrides_destroy

> String assignment_overrides_destroy(assignment_id, course_id, id)
Delete an assignment override

Deletes an override and returns its former details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assignment_overrides_group_alias

> assignment_overrides_group_alias(assignment_id, group_id)
Redirect to the assignment override for a group

Responds with a redirect to the override for the given group, if any (404 otherwise).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**group_id** | **String** | Scope response to group_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assignment_overrides_index

> models::AssignmentOverride assignment_overrides_index(assignment_id, course_id)
List assignment overrides

Returns the paginated list of overrides for this assignment that target sections/groups/students visible to the current user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |

### Return type

[**models::AssignmentOverride**](AssignmentOverride.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assignment_overrides_section_alias

> assignment_overrides_section_alias(assignment_id, course_section_id)
Redirect to the assignment override for a section

Responds with a redirect to the override for the given section, if any (404 otherwise).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_section_id** | **String** | Scope response to course_section_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assignment_overrides_show

> String assignment_overrides_show(assignment_id, course_id, id)
Get a single assignment override

Returns details of the the override with the given id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assignment_overrides_update

> String assignment_overrides_update(assignment_id, course_id, id, assignment_overrides_update_request)
Update an assignment override

All current overridden values must be supplied if they are to be retained; e.g. if due_at was overridden, but this PUT omits a value for due_at, due_at will no longer be overridden. If the override is adhoc and student_ids is not supplied, the target override set is unchanged. Target override sets cannot be changed for group or section overrides.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |
**assignment_overrides_update_request** | Option<[**AssignmentOverridesUpdateRequest**](AssignmentOverridesUpdateRequest.md)> | Request body parameters  **Example Request:** ``` curl 'https://<canvas>/api/v1/courses/1/assignments/2/overrides/3.json' \\      -X PUT \\      -F 'assignment_override[title]=Fred Flinstone' \\      -F 'assignment_override[due_at]=2012-10-08T21:00:00Z' \\      -H \"Authorization: Bearer <token>\" ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

