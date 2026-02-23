# \PeerReviewsApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**peer_reviews_api_allocate**](PeerReviewsApiApi.md#peer_reviews_api_allocate) | **POST** /courses/{course_id}/assignments/{assignment_id}/allocate | Allocate Peer Review
[**peer_reviews_api_create_for_courses**](PeerReviewsApiApi.md#peer_reviews_api_create_for_courses) | **POST** /courses/{course_id}/assignments/{assignment_id}/submissions/{submission_id}/peer_reviews | Create Peer Review
[**peer_reviews_api_create_other**](PeerReviewsApiApi.md#peer_reviews_api_create_other) | **POST** /sections/{section_id}/assignments/{assignment_id}/submissions/{submission_id}/peer_reviews | Create Peer Review
[**peer_reviews_api_destroy_for_courses**](PeerReviewsApiApi.md#peer_reviews_api_destroy_for_courses) | **DELETE** /courses/{course_id}/assignments/{assignment_id}/submissions/{submission_id}/peer_reviews | Delete Peer Review
[**peer_reviews_api_destroy_other**](PeerReviewsApiApi.md#peer_reviews_api_destroy_other) | **DELETE** /sections/{section_id}/assignments/{assignment_id}/submissions/{submission_id}/peer_reviews | Delete Peer Review
[**peer_reviews_api_index_for_courses**](PeerReviewsApiApi.md#peer_reviews_api_index_for_courses) | **GET** /courses/{course_id}/assignments/{assignment_id}/peer_reviews | Get all Peer Reviews
[**peer_reviews_api_index_for_courses2**](PeerReviewsApiApi.md#peer_reviews_api_index_for_courses2) | **GET** /courses/{course_id}/assignments/{assignment_id}/submissions/{submission_id}/peer_reviews | Get all Peer Reviews
[**peer_reviews_api_index_other**](PeerReviewsApiApi.md#peer_reviews_api_index_other) | **GET** /sections/{section_id}/assignments/{assignment_id}/peer_reviews | Get all Peer Reviews
[**peer_reviews_api_index_other2**](PeerReviewsApiApi.md#peer_reviews_api_index_other2) | **GET** /sections/{section_id}/assignments/{assignment_id}/submissions/{submission_id}/peer_reviews | Get all Peer Reviews



## peer_reviews_api_allocate

> String peer_reviews_api_allocate(assignment_id, course_id)
Allocate Peer Review

Allocates a submission for the current user to peer review

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## peer_reviews_api_create_for_courses

> String peer_reviews_api_create_for_courses(assignment_id, course_id, submission_id, peer_reviews_api_create_for_courses_request)
Create Peer Review

Create a peer review for the assignment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**submission_id** | **String** | Scope response to submission_id | [required] |
**peer_reviews_api_create_for_courses_request** | Option<[**PeerReviewsApiCreateForCoursesRequest**](PeerReviewsApiCreateForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## peer_reviews_api_create_other

> String peer_reviews_api_create_other(section_id, assignment_id, submission_id, peer_reviews_api_create_for_courses_request)
Create Peer Review

Create a peer review for the assignment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**section_id** | **String** | ID of the section | [required] |
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**submission_id** | **String** | Scope response to submission_id | [required] |
**peer_reviews_api_create_for_courses_request** | Option<[**PeerReviewsApiCreateForCoursesRequest**](PeerReviewsApiCreateForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## peer_reviews_api_destroy_for_courses

> String peer_reviews_api_destroy_for_courses(assignment_id, course_id, submission_id, user_id)
Delete Peer Review

Delete a peer review for the assignment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**submission_id** | **String** | Scope response to submission_id | [required] |
**user_id** | **i32** | user_id to delete as reviewer on this assignment | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## peer_reviews_api_destroy_other

> String peer_reviews_api_destroy_other(section_id, assignment_id, submission_id, user_id)
Delete Peer Review

Delete a peer review for the assignment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**section_id** | **String** | ID of the section | [required] |
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**submission_id** | **String** | Scope response to submission_id | [required] |
**user_id** | **i32** | user_id to delete as reviewer on this assignment | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## peer_reviews_api_index_for_courses

> serde_json::Value peer_reviews_api_index_for_courses(assignment_id, course_id, include)
Get all Peer Reviews

Get a list of all Peer Reviews for this assignment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"submission_comments\"|\"user\"] Associations to include with the peer review. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## peer_reviews_api_index_for_courses2

> serde_json::Value peer_reviews_api_index_for_courses2(submission_id, assignment_id, course_id, include)
Get all Peer Reviews

Get a list of all Peer Reviews for this assignment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**submission_id** | **String** | ID of the submission | [required] |
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"submission_comments\"|\"user\"] Associations to include with the peer review. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## peer_reviews_api_index_other

> serde_json::Value peer_reviews_api_index_other(section_id, assignment_id, include)
Get all Peer Reviews

Get a list of all Peer Reviews for this assignment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**section_id** | **String** | ID of the section | [required] |
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"submission_comments\"|\"user\"] Associations to include with the peer review. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## peer_reviews_api_index_other2

> serde_json::Value peer_reviews_api_index_other2(submission_id, section_id, assignment_id, include)
Get all Peer Reviews

Get a list of all Peer Reviews for this assignment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**submission_id** | **String** | ID of the submission | [required] |
**section_id** | **String** | ID of the section | [required] |
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"submission_comments\"|\"user\"] Associations to include with the peer review. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

