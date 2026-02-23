# \AssignmentsApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assignments_api_bulk_update**](AssignmentsApiApi.md#assignments_api_bulk_update) | **PUT** /courses/{course_id}/assignments/bulk_update | Bulk update assignment dates
[**assignments_api_check_allocation_conversion**](AssignmentsApiApi.md#assignments_api_check_allocation_conversion) | **GET** /courses/{course_id}/assignments/{assignment_id}/check_allocation_conversion | Check allocation conversion
[**assignments_api_create**](AssignmentsApiApi.md#assignments_api_create) | **POST** /courses/{course_id}/assignments | Create an assignment
[**assignments_api_duplicate**](AssignmentsApiApi.md#assignments_api_duplicate) | **POST** /courses/{course_id}/assignments/{assignment_id}/duplicate | Duplicate assignment
[**assignments_api_index**](AssignmentsApiApi.md#assignments_api_index) | **GET** /courses/{course_id}/assignment_groups/{assignment_group_id}/assignments | List assignments
[**assignments_api_show**](AssignmentsApiApi.md#assignments_api_show) | **GET** /courses/{course_id}/assignments/{id} | Get a single assignment
[**assignments_api_student_group_members**](AssignmentsApiApi.md#assignments_api_student_group_members) | **GET** /courses/{course_id}/assignments/{assignment_id}/users/{user_id}/group_members | List group members for a student on an assignment
[**assignments_api_update**](AssignmentsApiApi.md#assignments_api_update) | **PUT** /courses/{course_id}/assignments/{id} | Edit an assignment
[**assignments_api_user_index**](AssignmentsApiApi.md#assignments_api_user_index) | **GET** /users/{user_id}/courses/{course_id}/assignments | List assignments for user



## assignments_api_bulk_update

> String assignments_api_bulk_update(course_id)
Bulk update assignment dates

Update due dates and availability dates for multiple assignments in a course.  Accepts a JSON array of objects containing two keys each: +id+, the assignment id, and +all_dates+, an array of +AssignmentDate+ structures containing the base and/or override dates for the assignment, as returned from the {api:AssignmentsApiController#index List assignments} endpoint with +include[]=all_dates+.  This endpoint cannot create or destroy assignment overrides; any existing assignment overrides that are not referenced in the arguments will be left alone. If an override is given, any dates that are not supplied with it will be defaulted. To clear a date, specify null explicitly.  All referenced assignments will be validated before any are saved. A list of errors will be returned if any provided dates are invalid, and no changes will be saved.  The bulk update is performed in a background job, use the {api:ProgressController#show Progress API} to check its status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assignments_api_check_allocation_conversion

> serde_json::Value assignments_api_check_allocation_conversion(assignment_id, course_id)
Check allocation conversion

Returns a list of allocation objects that would be converted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assignments_api_create

> String assignments_api_create(course_id, assignments_api_create_request)
Create an assignment

Create a new assignment for this course. The assignment is created in the active state.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**assignments_api_create_request** | Option<[**AssignmentsApiCreateRequest**](AssignmentsApiCreateRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assignments_api_duplicate

> String assignments_api_duplicate(assignment_id, course_id, assignments_api_duplicate_request)
Duplicate assignment

Duplicate an assignment and return a json based on result_type argument.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**assignments_api_duplicate_request** | Option<[**AssignmentsApiDuplicateRequest**](AssignmentsApiDuplicateRequest.md)> | Request body parameters  **Example Request:** ``` curl -X POST -H 'Authorization: Bearer <token>' \\ https://<canvas>/api/v1/courses/123/assignments/123/duplicate ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assignments_api_index

> models::Assignment assignments_api_index(assignment_group_id, course_id, include, search_term, override_assignment_dates, needs_grading_count_by_section, bucket, assignment_ids, order_by, post_to_sis, new_quizzes, page, per_page)
List assignments

Returns the paginated list of assignments for the current course or assignment group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_group_id** | **String** | ID of the assignment group | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"submission\"|\"assignment_visibility\"|\"all_dates\"|\"overrides\"|\"observed_users\"|\"can_edit\"|\"score_statistics\"|\"ab_guid\"] Optional information to include with each assignment: submission:: The current user's current +Submission+ assignment_visibility:: An array of ids of students who can see the assignment all_dates:: An array of +AssignmentDate+ structures, one for each override, and also a +base+ if the assignment has an \"Everyone\" / \"Everyone Else\" date overrides:: An array of +AssignmentOverride+ structures observed_users:: An array of submissions for observed users can_edit:: an extra Boolean value will be included with each +Assignment+ (and +AssignmentDate+ if +all_dates+ is supplied) to indicate whether the caller can edit the assignment or date. Moderated grading and closed grading periods may restrict a user's ability to edit an assignment. score_statistics:: An object containing min, max, and mean score on this assignment. This will not be included for students if there are less than 5 graded assignments or if disabled by the instructor. Only valid if 'submission' is also included. ab_guid:: An array of guid strings for academic benchmarks |  |
**search_term** | Option<**String**> | The partial title of the assignments to match and return. |  |
**override_assignment_dates** | Option<**bool**> | Apply assignment overrides for each assignment, defaults to true. |  |
**needs_grading_count_by_section** | Option<**bool**> | Split up \"needs_grading_count\" by sections into the \"needs_grading_count_by_section\" key, defaults to false |  |
**bucket** | Option<**String**> | If included, only return certain assignments depending on due date and submission status. |  |
**assignment_ids** | Option<[**Vec<String>**](String.md)> | if set, return only assignments specified |  |
**order_by** | Option<**String**> | Determines the order of the assignments. Defaults to \"position\". |  |
**post_to_sis** | Option<**bool**> | Return only assignments that have post_to_sis set or not set. |  |
**new_quizzes** | Option<**bool**> | Return only New Quizzes assignments |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::Assignment**](Assignment.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assignments_api_show

> String assignments_api_show(course_id, id, include, override_assignment_dates, needs_grading_count_by_section, all_dates)
Get a single assignment

Returns the assignment with the given id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"submission\"|\"assignment_visibility\"|\"overrides\"|\"observed_users\"|\"can_edit\"|\"score_statistics\"|\"ab_guid\"|\"peer_review\"] Associations to include with the assignment. The \"assignment_visibility\" option requires that the Differentiated Assignments course feature be turned on. If \"observed_users\" is passed, submissions for observed users will also be included. For \"score_statistics\" to be included, the \"submission\" option must also be set. The \"peer_review\" option requires that the Peer Review Allocation and Grading course feature be turned on. |  |
**override_assignment_dates** | Option<**bool**> | Apply assignment overrides to the assignment, defaults to true. |  |
**needs_grading_count_by_section** | Option<**bool**> | Split up \"needs_grading_count\" by sections into the \"needs_grading_count_by_section\" key, defaults to false |  |
**all_dates** | Option<**bool**> | All dates associated with the assignment, if applicable |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assignments_api_student_group_members

> models::BasicUser assignments_api_student_group_members(assignment_id, course_id, user_id)
List group members for a student on an assignment

Returns student ids and names for the group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

[**models::BasicUser**](BasicUser.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assignments_api_update

> String assignments_api_update(course_id, id, assignments_api_update_request)
Edit an assignment

Modify an existing assignment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |
**assignments_api_update_request** | Option<[**AssignmentsApiUpdateRequest**](AssignmentsApiUpdateRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assignments_api_user_index

> assignments_api_user_index(course_id, user_id, page, per_page)
List assignments for user

Returns the paginated list of assignments for the specified user if the current user has rights to view. See {api:AssignmentsApiController#index List assignments} for valid arguments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

