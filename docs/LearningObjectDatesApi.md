# \LearningObjectDatesApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**learning_object_dates_show_for_courses**](LearningObjectDatesApi.md#learning_object_dates_show_for_courses) | **GET** /courses/{course_id}/modules/{context_module_id}/date_details | Get a learning object's date information
[**learning_object_dates_show_for_courses2**](LearningObjectDatesApi.md#learning_object_dates_show_for_courses2) | **GET** /courses/{course_id}/assignments/{assignment_id}/date_details | Get a learning object's date information
[**learning_object_dates_show_for_courses3**](LearningObjectDatesApi.md#learning_object_dates_show_for_courses3) | **GET** /courses/{course_id}/quizzes/{quiz_id}/date_details | Get a learning object's date information
[**learning_object_dates_show_for_courses4**](LearningObjectDatesApi.md#learning_object_dates_show_for_courses4) | **GET** /courses/{course_id}/discussion_topics/{discussion_topic_id}/date_details | Get a learning object's date information
[**learning_object_dates_show_for_courses5**](LearningObjectDatesApi.md#learning_object_dates_show_for_courses5) | **GET** /courses/{course_id}/pages/{url_or_id}/date_details | Get a learning object's date information
[**learning_object_dates_show_for_courses6**](LearningObjectDatesApi.md#learning_object_dates_show_for_courses6) | **GET** /courses/{course_id}/files/{attachment_id}/date_details | Get a learning object's date information
[**learning_object_dates_update_for_courses**](LearningObjectDatesApi.md#learning_object_dates_update_for_courses) | **PUT** /courses/{course_id}/assignments/{assignment_id}/date_details | Update a learning object's date information
[**learning_object_dates_update_for_courses2**](LearningObjectDatesApi.md#learning_object_dates_update_for_courses2) | **PUT** /courses/{course_id}/quizzes/{quiz_id}/date_details | Update a learning object's date information
[**learning_object_dates_update_for_courses3**](LearningObjectDatesApi.md#learning_object_dates_update_for_courses3) | **PUT** /courses/{course_id}/discussion_topics/{discussion_topic_id}/date_details | Update a learning object's date information
[**learning_object_dates_update_for_courses4**](LearningObjectDatesApi.md#learning_object_dates_update_for_courses4) | **PUT** /courses/{course_id}/pages/{url_or_id}/date_details | Update a learning object's date information
[**learning_object_dates_update_for_courses5**](LearningObjectDatesApi.md#learning_object_dates_update_for_courses5) | **PUT** /courses/{course_id}/files/{attachment_id}/date_details | Update a learning object's date information



## learning_object_dates_show_for_courses

> String learning_object_dates_show_for_courses(context_module_id, course_id, include, exclude, page, per_page)
Get a learning object's date information

Get a learning object's date-related information, including due date, availability dates, override status, and a paginated list of all assignment overrides for the item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_module_id** | **String** | Scope response to context_module_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [Array] Array of strings indicating what additional data to include in the response. Valid values: - \"peer_review\": includes peer review sub assignment information and overrides in the response.   Requires the peer_review_allocation_and_grading feature flag to be enabled. - \"child_peer_review_override_dates\": each assignment override will include a peer_review_dates   field containing the matched peer review override data (id, due_at, unlock_at, lock_at)   for that override. The field will be present as null if no matching peer review override exists. |  |
**exclude** | Option<[**Vec<String>**](String.md)> | [Array] Array of strings indicating what data to exclude from the response. Valid values: - \"peer_review_overrides\": when include[]=peer_review is also specified, the   peer_review_sub_assignment object will not include the overrides array, reducing the   response payload size. This is useful when using include[]=child_peer_review_override_dates   since the peer review override data is already embedded in the parent assignment overrides. - \"child_override_due_dates\": prevents the sub_assignment_due_dates field from being included   in assignment override responses, even when discussion checkpoints are enabled. This reduces   response payload size when checkpoint due date information is not needed. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## learning_object_dates_show_for_courses2

> String learning_object_dates_show_for_courses2(assignment_id, course_id, include, exclude, page, per_page)
Get a learning object's date information

Get a learning object's date-related information, including due date, availability dates, override status, and a paginated list of all assignment overrides for the item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | ID of the assignment | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [Array] Array of strings indicating what additional data to include in the response. Valid values: - \"peer_review\": includes peer review sub assignment information and overrides in the response.   Requires the peer_review_allocation_and_grading feature flag to be enabled. - \"child_peer_review_override_dates\": each assignment override will include a peer_review_dates   field containing the matched peer review override data (id, due_at, unlock_at, lock_at)   for that override. The field will be present as null if no matching peer review override exists. |  |
**exclude** | Option<[**Vec<String>**](String.md)> | [Array] Array of strings indicating what data to exclude from the response. Valid values: - \"peer_review_overrides\": when include[]=peer_review is also specified, the   peer_review_sub_assignment object will not include the overrides array, reducing the   response payload size. This is useful when using include[]=child_peer_review_override_dates   since the peer review override data is already embedded in the parent assignment overrides. - \"child_override_due_dates\": prevents the sub_assignment_due_dates field from being included   in assignment override responses, even when discussion checkpoints are enabled. This reduces   response payload size when checkpoint due date information is not needed. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## learning_object_dates_show_for_courses3

> String learning_object_dates_show_for_courses3(quiz_id, course_id, include, exclude, page, per_page)
Get a learning object's date information

Get a learning object's date-related information, including due date, availability dates, override status, and a paginated list of all assignment overrides for the item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**quiz_id** | **String** | ID of the quiz | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [Array] Array of strings indicating what additional data to include in the response. Valid values: - \"peer_review\": includes peer review sub assignment information and overrides in the response.   Requires the peer_review_allocation_and_grading feature flag to be enabled. - \"child_peer_review_override_dates\": each assignment override will include a peer_review_dates   field containing the matched peer review override data (id, due_at, unlock_at, lock_at)   for that override. The field will be present as null if no matching peer review override exists. |  |
**exclude** | Option<[**Vec<String>**](String.md)> | [Array] Array of strings indicating what data to exclude from the response. Valid values: - \"peer_review_overrides\": when include[]=peer_review is also specified, the   peer_review_sub_assignment object will not include the overrides array, reducing the   response payload size. This is useful when using include[]=child_peer_review_override_dates   since the peer review override data is already embedded in the parent assignment overrides. - \"child_override_due_dates\": prevents the sub_assignment_due_dates field from being included   in assignment override responses, even when discussion checkpoints are enabled. This reduces   response payload size when checkpoint due date information is not needed. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## learning_object_dates_show_for_courses4

> String learning_object_dates_show_for_courses4(discussion_topic_id, course_id, include, exclude, page, per_page)
Get a learning object's date information

Get a learning object's date-related information, including due date, availability dates, override status, and a paginated list of all assignment overrides for the item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**discussion_topic_id** | **String** | ID of the discussion topic | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [Array] Array of strings indicating what additional data to include in the response. Valid values: - \"peer_review\": includes peer review sub assignment information and overrides in the response.   Requires the peer_review_allocation_and_grading feature flag to be enabled. - \"child_peer_review_override_dates\": each assignment override will include a peer_review_dates   field containing the matched peer review override data (id, due_at, unlock_at, lock_at)   for that override. The field will be present as null if no matching peer review override exists. |  |
**exclude** | Option<[**Vec<String>**](String.md)> | [Array] Array of strings indicating what data to exclude from the response. Valid values: - \"peer_review_overrides\": when include[]=peer_review is also specified, the   peer_review_sub_assignment object will not include the overrides array, reducing the   response payload size. This is useful when using include[]=child_peer_review_override_dates   since the peer review override data is already embedded in the parent assignment overrides. - \"child_override_due_dates\": prevents the sub_assignment_due_dates field from being included   in assignment override responses, even when discussion checkpoints are enabled. This reduces   response payload size when checkpoint due date information is not needed. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## learning_object_dates_show_for_courses5

> String learning_object_dates_show_for_courses5(url_or_id, course_id, include, exclude, page, per_page)
Get a learning object's date information

Get a learning object's date-related information, including due date, availability dates, override status, and a paginated list of all assignment overrides for the item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url_or_id** | **String** | ID of the url or | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [Array] Array of strings indicating what additional data to include in the response. Valid values: - \"peer_review\": includes peer review sub assignment information and overrides in the response.   Requires the peer_review_allocation_and_grading feature flag to be enabled. - \"child_peer_review_override_dates\": each assignment override will include a peer_review_dates   field containing the matched peer review override data (id, due_at, unlock_at, lock_at)   for that override. The field will be present as null if no matching peer review override exists. |  |
**exclude** | Option<[**Vec<String>**](String.md)> | [Array] Array of strings indicating what data to exclude from the response. Valid values: - \"peer_review_overrides\": when include[]=peer_review is also specified, the   peer_review_sub_assignment object will not include the overrides array, reducing the   response payload size. This is useful when using include[]=child_peer_review_override_dates   since the peer review override data is already embedded in the parent assignment overrides. - \"child_override_due_dates\": prevents the sub_assignment_due_dates field from being included   in assignment override responses, even when discussion checkpoints are enabled. This reduces   response payload size when checkpoint due date information is not needed. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## learning_object_dates_show_for_courses6

> String learning_object_dates_show_for_courses6(attachment_id, course_id, include, exclude, page, per_page)
Get a learning object's date information

Get a learning object's date-related information, including due date, availability dates, override status, and a paginated list of all assignment overrides for the item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attachment_id** | **String** | ID of the attachment | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [Array] Array of strings indicating what additional data to include in the response. Valid values: - \"peer_review\": includes peer review sub assignment information and overrides in the response.   Requires the peer_review_allocation_and_grading feature flag to be enabled. - \"child_peer_review_override_dates\": each assignment override will include a peer_review_dates   field containing the matched peer review override data (id, due_at, unlock_at, lock_at)   for that override. The field will be present as null if no matching peer review override exists. |  |
**exclude** | Option<[**Vec<String>**](String.md)> | [Array] Array of strings indicating what data to exclude from the response. Valid values: - \"peer_review_overrides\": when include[]=peer_review is also specified, the   peer_review_sub_assignment object will not include the overrides array, reducing the   response payload size. This is useful when using include[]=child_peer_review_override_dates   since the peer review override data is already embedded in the parent assignment overrides. - \"child_override_due_dates\": prevents the sub_assignment_due_dates field from being included   in assignment override responses, even when discussion checkpoints are enabled. This reduces   response payload size when checkpoint due date information is not needed. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## learning_object_dates_update_for_courses

> learning_object_dates_update_for_courses(assignment_id, course_id, learning_object_dates_update_for_courses_request)
Update a learning object's date information

Updates date-related information for learning objects, including due date, availability dates, override status, and assignment overrides.  Returns 204 No Content response code if successful.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**learning_object_dates_update_for_courses_request** | Option<[**LearningObjectDatesUpdateForCoursesRequest**](LearningObjectDatesUpdateForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## learning_object_dates_update_for_courses2

> learning_object_dates_update_for_courses2(quiz_id, course_id, learning_object_dates_update_for_courses_request)
Update a learning object's date information

Updates date-related information for learning objects, including due date, availability dates, override status, and assignment overrides.  Returns 204 No Content response code if successful.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**quiz_id** | **String** | ID of the quiz | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**learning_object_dates_update_for_courses_request** | Option<[**LearningObjectDatesUpdateForCoursesRequest**](LearningObjectDatesUpdateForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## learning_object_dates_update_for_courses3

> learning_object_dates_update_for_courses3(discussion_topic_id, course_id, learning_object_dates_update_for_courses_request)
Update a learning object's date information

Updates date-related information for learning objects, including due date, availability dates, override status, and assignment overrides.  Returns 204 No Content response code if successful.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**discussion_topic_id** | **String** | ID of the discussion topic | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**learning_object_dates_update_for_courses_request** | Option<[**LearningObjectDatesUpdateForCoursesRequest**](LearningObjectDatesUpdateForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## learning_object_dates_update_for_courses4

> learning_object_dates_update_for_courses4(url_or_id, course_id, learning_object_dates_update_for_courses_request)
Update a learning object's date information

Updates date-related information for learning objects, including due date, availability dates, override status, and assignment overrides.  Returns 204 No Content response code if successful.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url_or_id** | **String** | ID of the url or | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**learning_object_dates_update_for_courses_request** | Option<[**LearningObjectDatesUpdateForCoursesRequest**](LearningObjectDatesUpdateForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## learning_object_dates_update_for_courses5

> learning_object_dates_update_for_courses5(attachment_id, course_id, learning_object_dates_update_for_courses_request)
Update a learning object's date information

Updates date-related information for learning objects, including due date, availability dates, override status, and assignment overrides.  Returns 204 No Content response code if successful.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attachment_id** | **String** | ID of the attachment | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**learning_object_dates_update_for_courses_request** | Option<[**LearningObjectDatesUpdateForCoursesRequest**](LearningObjectDatesUpdateForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

