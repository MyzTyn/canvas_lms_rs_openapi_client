# \MediaObjectsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**media_objects_index_for_courses**](MediaObjectsApi.md#media_objects_index_for_courses) | **GET** /courses/{course_id}/media_objects | List Media Objects
[**media_objects_index_for_courses2**](MediaObjectsApi.md#media_objects_index_for_courses2) | **GET** /courses/{course_id}/media_attachments | List Media Objects
[**media_objects_index_for_groups**](MediaObjectsApi.md#media_objects_index_for_groups) | **GET** /groups/{group_id}/media_objects | List Media Objects
[**media_objects_index_for_groups2**](MediaObjectsApi.md#media_objects_index_for_groups2) | **GET** /groups/{group_id}/media_attachments | List Media Objects
[**media_objects_index_other**](MediaObjectsApi.md#media_objects_index_other) | **GET** /media_objects | List Media Objects
[**media_objects_index_other2**](MediaObjectsApi.md#media_objects_index_other2) | **GET** /media_attachments | List Media Objects
[**media_objects_update_media_object_other**](MediaObjectsApi.md#media_objects_update_media_object_other) | **PUT** /media_objects/{media_object_id} | Update Media Object
[**media_objects_update_media_object_other2**](MediaObjectsApi.md#media_objects_update_media_object_other2) | **PUT** /media_attachments/{attachment_id} | Update Media Object



## media_objects_index_for_courses

> serde_json::Value media_objects_index_for_courses(course_id, sort, order, exclude, page, per_page)
List Media Objects

Returns media objects created by the user making the request. When using the second version, returns media objects associated with the given course.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | ID of the course | [required] |
**sort** | Option<**String**> | Field to sort on. Default is \"title\"  title:: sorts on user_entered_title if available, title if not.  created_at:: sorts on the object's creation time. |  |
**order** | Option<**String**> | Sort direction. Default is \"asc\" |  |
**exclude** | Option<[**Vec<String>**](String.md)> | [String, \"sources\"|\"tracks\"] Array of data to exclude. By excluding \"sources\" and \"tracks\", the api will not need to query kaltura, which greatly speeds up its response.  sources:: Do not query kaltura for media_sources tracks:: Do not query kaltura for media_tracks |  |
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


## media_objects_index_for_courses2

> serde_json::Value media_objects_index_for_courses2(course_id, sort, order, exclude, page, per_page)
List Media Objects

Returns media objects created by the user making the request. When using the second version, returns media objects associated with the given course.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | ID of the course | [required] |
**sort** | Option<**String**> | Field to sort on. Default is \"title\"  title:: sorts on user_entered_title if available, title if not.  created_at:: sorts on the object's creation time. |  |
**order** | Option<**String**> | Sort direction. Default is \"asc\" |  |
**exclude** | Option<[**Vec<String>**](String.md)> | [String, \"sources\"|\"tracks\"] Array of data to exclude. By excluding \"sources\" and \"tracks\", the api will not need to query kaltura, which greatly speeds up its response.  sources:: Do not query kaltura for media_sources tracks:: Do not query kaltura for media_tracks |  |
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


## media_objects_index_for_groups

> serde_json::Value media_objects_index_for_groups(group_id, sort, order, exclude, page, per_page)
List Media Objects

Returns media objects created by the user making the request. When using the second version, returns media objects associated with the given course.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**sort** | Option<**String**> | Field to sort on. Default is \"title\"  title:: sorts on user_entered_title if available, title if not.  created_at:: sorts on the object's creation time. |  |
**order** | Option<**String**> | Sort direction. Default is \"asc\" |  |
**exclude** | Option<[**Vec<String>**](String.md)> | [String, \"sources\"|\"tracks\"] Array of data to exclude. By excluding \"sources\" and \"tracks\", the api will not need to query kaltura, which greatly speeds up its response.  sources:: Do not query kaltura for media_sources tracks:: Do not query kaltura for media_tracks |  |
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


## media_objects_index_for_groups2

> serde_json::Value media_objects_index_for_groups2(group_id, sort, order, exclude, page, per_page)
List Media Objects

Returns media objects created by the user making the request. When using the second version, returns media objects associated with the given course.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**sort** | Option<**String**> | Field to sort on. Default is \"title\"  title:: sorts on user_entered_title if available, title if not.  created_at:: sorts on the object's creation time. |  |
**order** | Option<**String**> | Sort direction. Default is \"asc\" |  |
**exclude** | Option<[**Vec<String>**](String.md)> | [String, \"sources\"|\"tracks\"] Array of data to exclude. By excluding \"sources\" and \"tracks\", the api will not need to query kaltura, which greatly speeds up its response.  sources:: Do not query kaltura for media_sources tracks:: Do not query kaltura for media_tracks |  |
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


## media_objects_index_other

> serde_json::Value media_objects_index_other(sort, order, exclude, page, per_page)
List Media Objects

Returns media objects created by the user making the request. When using the second version, returns media objects associated with the given course.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sort** | Option<**String**> | Field to sort on. Default is \"title\"  title:: sorts on user_entered_title if available, title if not.  created_at:: sorts on the object's creation time. |  |
**order** | Option<**String**> | Sort direction. Default is \"asc\" |  |
**exclude** | Option<[**Vec<String>**](String.md)> | [String, \"sources\"|\"tracks\"] Array of data to exclude. By excluding \"sources\" and \"tracks\", the api will not need to query kaltura, which greatly speeds up its response.  sources:: Do not query kaltura for media_sources tracks:: Do not query kaltura for media_tracks |  |
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


## media_objects_index_other2

> serde_json::Value media_objects_index_other2(sort, order, exclude, page, per_page)
List Media Objects

Returns media objects created by the user making the request. When using the second version, returns media objects associated with the given course.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sort** | Option<**String**> | Field to sort on. Default is \"title\"  title:: sorts on user_entered_title if available, title if not.  created_at:: sorts on the object's creation time. |  |
**order** | Option<**String**> | Sort direction. Default is \"asc\" |  |
**exclude** | Option<[**Vec<String>**](String.md)> | [String, \"sources\"|\"tracks\"] Array of data to exclude. By excluding \"sources\" and \"tracks\", the api will not need to query kaltura, which greatly speeds up its response.  sources:: Do not query kaltura for media_sources tracks:: Do not query kaltura for media_tracks |  |
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


## media_objects_update_media_object_other

> media_objects_update_media_object_other(media_object_id, media_objects_update_media_object_other_request)
Update Media Object

Updates the title of a media object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**media_object_id** | **String** | Scope response to media_object_id | [required] |
**media_objects_update_media_object_other_request** | Option<[**MediaObjectsUpdateMediaObjectOtherRequest**](MediaObjectsUpdateMediaObjectOtherRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## media_objects_update_media_object_other2

> media_objects_update_media_object_other2(attachment_id, media_objects_update_media_object_other_request)
Update Media Object

Updates the title of a media object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attachment_id** | **String** | ID of the attachment | [required] |
**media_objects_update_media_object_other_request** | Option<[**MediaObjectsUpdateMediaObjectOtherRequest**](MediaObjectsUpdateMediaObjectOtherRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

