# \PlannerNotesApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**planner_notes_create**](PlannerNotesApi.md#planner_notes_create) | **POST** /planner_notes | Create a planner note
[**planner_notes_destroy**](PlannerNotesApi.md#planner_notes_destroy) | **DELETE** /planner_notes/{id} | Delete a planner note
[**planner_notes_index**](PlannerNotesApi.md#planner_notes_index) | **GET** /planner_notes | List planner notes
[**planner_notes_show**](PlannerNotesApi.md#planner_notes_show) | **GET** /planner_notes/{id} | Show a planner note
[**planner_notes_update**](PlannerNotesApi.md#planner_notes_update) | **PUT** /planner_notes/{id} | Update a planner note



## planner_notes_create

> String planner_notes_create(planner_notes_create_request)
Create a planner note

Create a planner note for the current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**planner_notes_create_request** | Option<[**PlannerNotesCreateRequest**](PlannerNotesCreateRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## planner_notes_destroy

> String planner_notes_destroy(id)
Delete a planner note

Delete a planner note for the current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## planner_notes_index

> models::PlannerNote planner_notes_index(start_date, end_date, context_codes)
List planner notes

Retrieve the paginated list of planner notes  Retrieve planner note for a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_date** | Option<**String**> | Only return notes with todo dates since the start_date (inclusive). No default. The value should be formatted as: yyyy-mm-dd or ISO 8601 YYYY-MM-DDTHH:MM:SSZ. |  |
**end_date** | Option<**String**> | Only return notes with todo dates before the end_date (inclusive). No default. The value should be formatted as: yyyy-mm-dd or ISO 8601 YYYY-MM-DDTHH:MM:SSZ. If end_date and start_date are both specified and equivalent, then only notes with todo dates on that day are returned. |  |
**context_codes** | Option<[**Vec<String>**](String.md)> | [String] List of context codes of courses whose notes you want to see. If not specified, defaults to all contexts that the user belongs to. The format of this field is the context type, followed by an underscore, followed by the context id. For example: course_42 Including a code matching the user's own context code (e.g. user_1) will include notes that are not associated with any particular course. |  |

### Return type

[**models::PlannerNote**](PlannerNote.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## planner_notes_show

> String planner_notes_show(id)
Show a planner note

Retrieve a planner note for the current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## planner_notes_update

> String planner_notes_update(id, planner_notes_update_request)
Update a planner note

Update a planner note for the current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**planner_notes_update_request** | Option<[**PlannerNotesUpdateRequest**](PlannerNotesUpdateRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

