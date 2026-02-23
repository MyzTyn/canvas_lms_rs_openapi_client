# \AssignmentGroupsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assignment_groups_index**](AssignmentGroupsApi.md#assignment_groups_index) | **GET** /courses/{course_id}/assignment_groups | List assignment groups



## assignment_groups_index

> models::AssignmentGroup assignment_groups_index(course_id, include, assignment_ids, exclude_assignment_submission_types, override_assignment_dates, grading_period_id, scope_assignments_to_student, page, per_page)
List assignment groups

Returns the paginated list of assignment groups for the current context. The returned groups are sorted by their position field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"assignments\"|\"discussion_topic\"|\"all_dates\"|\"assignment_visibility\"|\"overrides\"|\"submission\"|\"observed_users\"|\"can_edit\"|\"score_statistics\"|\"peer_review\"] Associations to include with the group. \"discussion_topic\", \"all_dates\", \"can_edit\", \"assignment_visibility\" & \"submission\" are only valid if \"assignments\" is also included. \"score_statistics\" requires that the \"assignments\" and \"submission\" options are included. The \"assignment_visibility\" option additionally requires that the Differentiated Assignments course feature be turned on. If \"observed_users\" is passed along with \"assignments\" and \"submission\", submissions for observed users will also be included as an array. The \"peer_review\" option requires that the Peer Review Grading course feature be turned on and that \"assignments\" is included. |  |
**assignment_ids** | Option<[**Vec<String>**](String.md)> | [String] If \"assignments\" are included, optionally return only assignments having their ID in this array. This argument may also be passed as a comma separated string. |  |
**exclude_assignment_submission_types** | Option<[**Vec<String>**](String.md)> | [String, \"online_quiz\"|\"discussion_topic\"|\"wiki_page\"|\"external_tool\"] If \"assignments\" are included, those with the specified submission types will be excluded from the assignment groups. |  |
**override_assignment_dates** | Option<**bool**> | Apply assignment overrides for each assignment, defaults to true. |  |
**grading_period_id** | Option<**i32**> | The id of the grading period in which assignment groups are being requested (Requires grading periods to exist.) |  |
**scope_assignments_to_student** | Option<**bool**> | If true, all assignments returned will apply to the current user in the specified grading period. If assignments apply to other students in the specified grading period, but not the current user, they will not be returned. (Requires the grading_period_id argument and grading periods to exist. In addition, the current user must be a student.) |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::AssignmentGroup**](AssignmentGroup.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

