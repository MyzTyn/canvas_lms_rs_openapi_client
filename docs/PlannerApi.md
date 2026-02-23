# \PlannerApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**planner_index_for_users**](PlannerApi.md#planner_index_for_users) | **GET** /users/{user_id}/planner/items | List planner items
[**planner_index_other**](PlannerApi.md#planner_index_other) | **GET** /planner/items | List planner items



## planner_index_for_users

> planner_index_for_users(user_id, start_date, end_date, context_codes, observed_user_id, filter, page, per_page)
List planner items

Retrieve the paginated list of objects to be shown on the planner for the current user with the associated planner override to override an item's visibility if set.  Planner items for a student may also be retrieved by a linked observer. Use the path that accepts a user_id and supply the student's id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |
**start_date** | Option<**String**> | Only return items starting from the given date. The value should be formatted as: yyyy-mm-dd or ISO 8601 YYYY-MM-DDTHH:MM:SSZ. |  |
**end_date** | Option<**String**> | Only return items up to the given date. The value should be formatted as: yyyy-mm-dd or ISO 8601 YYYY-MM-DDTHH:MM:SSZ. |  |
**context_codes** | Option<[**Vec<String>**](String.md)> | [String] List of context codes of courses and/or groups whose items you want to see. If not specified, defaults to all contexts associated to the current user. Note that concluded courses will be ignored unless specified in the includes[] parameter. The format of this field is the context type, followed by an underscore, followed by the context id. For example: course_42, group_123 |  |
**observed_user_id** | Option<**String**> | Return planner items for the given observed user. Must be accompanied by context_codes[]. The user making the request must be observing the observed user in all the courses specified by context_codes[]. |  |
**filter** | Option<**String**> | Only return items that are completed (includes items with planner_override.marked_complete = true or submitted assignments) |  |
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


## planner_index_other

> planner_index_other(start_date, end_date, context_codes, observed_user_id, filter, page, per_page)
List planner items

Retrieve the paginated list of objects to be shown on the planner for the current user with the associated planner override to override an item's visibility if set.  Planner items for a student may also be retrieved by a linked observer. Use the path that accepts a user_id and supply the student's id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_date** | Option<**String**> | Only return items starting from the given date. The value should be formatted as: yyyy-mm-dd or ISO 8601 YYYY-MM-DDTHH:MM:SSZ. |  |
**end_date** | Option<**String**> | Only return items up to the given date. The value should be formatted as: yyyy-mm-dd or ISO 8601 YYYY-MM-DDTHH:MM:SSZ. |  |
**context_codes** | Option<[**Vec<String>**](String.md)> | [String] List of context codes of courses and/or groups whose items you want to see. If not specified, defaults to all contexts associated to the current user. Note that concluded courses will be ignored unless specified in the includes[] parameter. The format of this field is the context type, followed by an underscore, followed by the context id. For example: course_42, group_123 |  |
**observed_user_id** | Option<**String**> | Return planner items for the given observed user. Must be accompanied by context_codes[]. The user making the request must be observing the observed user in all the courses specified by context_codes[]. |  |
**filter** | Option<**String**> | Only return items that are completed (includes items with planner_override.marked_complete = true or submitted assignments) |  |
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

