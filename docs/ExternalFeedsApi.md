# \ExternalFeedsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**external_feeds_create_for_courses**](ExternalFeedsApi.md#external_feeds_create_for_courses) | **POST** /courses/{course_id}/external_feeds | Create an external feed
[**external_feeds_create_for_groups**](ExternalFeedsApi.md#external_feeds_create_for_groups) | **POST** /groups/{group_id}/external_feeds | Create an external feed
[**external_feeds_destroy_for_courses**](ExternalFeedsApi.md#external_feeds_destroy_for_courses) | **DELETE** /courses/{course_id}/external_feeds/{external_feed_id} | Delete an external feed
[**external_feeds_destroy_for_groups**](ExternalFeedsApi.md#external_feeds_destroy_for_groups) | **DELETE** /groups/{group_id}/external_feeds/{external_feed_id} | Delete an external feed
[**external_feeds_index_for_courses**](ExternalFeedsApi.md#external_feeds_index_for_courses) | **GET** /courses/{course_id}/external_feeds | List external feeds
[**external_feeds_index_for_groups**](ExternalFeedsApi.md#external_feeds_index_for_groups) | **GET** /groups/{group_id}/external_feeds | List external feeds



## external_feeds_create_for_courses

> String external_feeds_create_for_courses(course_id, external_feeds_create_for_courses_request)
Create an external feed

Create a new external feed for the course or group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**external_feeds_create_for_courses_request** | Option<[**ExternalFeedsCreateForCoursesRequest**](ExternalFeedsCreateForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## external_feeds_create_for_groups

> String external_feeds_create_for_groups(group_id, external_feeds_create_for_courses_request)
Create an external feed

Create a new external feed for the course or group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**external_feeds_create_for_courses_request** | Option<[**ExternalFeedsCreateForCoursesRequest**](ExternalFeedsCreateForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## external_feeds_destroy_for_courses

> String external_feeds_destroy_for_courses(course_id, external_feed_id)
Delete an external feed

Deletes the external feed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**external_feed_id** | **String** | Scope response to external_feed_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## external_feeds_destroy_for_groups

> String external_feeds_destroy_for_groups(group_id, external_feed_id)
Delete an external feed

Deletes the external feed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**external_feed_id** | **String** | Scope response to external_feed_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## external_feeds_index_for_courses

> serde_json::Value external_feeds_index_for_courses(course_id, page, per_page)
List external feeds

Returns the paginated list of External Feeds this course or group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## external_feeds_index_for_groups

> serde_json::Value external_feeds_index_for_groups(group_id, page, per_page)
List external feeds

Returns the paginated list of External Feeds this course or group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

