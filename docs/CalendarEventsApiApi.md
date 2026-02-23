# \CalendarEventsApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**calendar_events_api_create**](CalendarEventsApiApi.md#calendar_events_api_create) | **POST** /calendar_events | Create a calendar event
[**calendar_events_api_destroy**](CalendarEventsApiApi.md#calendar_events_api_destroy) | **DELETE** /calendar_events/{id} | Delete a calendar event
[**calendar_events_api_get_course_timetable**](CalendarEventsApiApi.md#calendar_events_api_get_course_timetable) | **GET** /courses/{course_id}/calendar_events/timetable | Get course timetable
[**calendar_events_api_index**](CalendarEventsApiApi.md#calendar_events_api_index) | **GET** /calendar_events | List calendar events
[**calendar_events_api_reserve_other**](CalendarEventsApiApi.md#calendar_events_api_reserve_other) | **POST** /calendar_events/{id}/reservations | Reserve a time slot
[**calendar_events_api_reserve_other2**](CalendarEventsApiApi.md#calendar_events_api_reserve_other2) | **POST** /calendar_events/{id}/reservations/{participant_id} | Reserve a time slot
[**calendar_events_api_save_enabled_account_calendars**](CalendarEventsApiApi.md#calendar_events_api_save_enabled_account_calendars) | **POST** /calendar_events/save_enabled_account_calendars | Save enabled account calendars
[**calendar_events_api_set_course_timetable**](CalendarEventsApiApi.md#calendar_events_api_set_course_timetable) | **POST** /courses/{course_id}/calendar_events/timetable | Set a course timetable
[**calendar_events_api_set_course_timetable_events**](CalendarEventsApiApi.md#calendar_events_api_set_course_timetable_events) | **POST** /courses/{course_id}/calendar_events/timetable_events | Create or update events directly for a course timetable
[**calendar_events_api_show**](CalendarEventsApiApi.md#calendar_events_api_show) | **GET** /calendar_events/{id} | Get a single calendar event or assignment
[**calendar_events_api_update**](CalendarEventsApiApi.md#calendar_events_api_update) | **PUT** /calendar_events/{id} | Update a calendar event
[**calendar_events_api_user_index**](CalendarEventsApiApi.md#calendar_events_api_user_index) | **GET** /users/{user_id}/calendar_events | List calendar events for a user



## calendar_events_api_create

> calendar_events_api_create(calendar_events_api_create_request)
Create a calendar event

Create and return a new calendar event

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**calendar_events_api_create_request** | Option<[**CalendarEventsApiCreateRequest**](CalendarEventsApiCreateRequest.md)> | Request body parameters  **Example Request:** ``` curl 'https://<canvas>/api/v1/calendar_events.json' \\      -X POST \\      -F 'calendar_event[context_code]=course_123' \\      -F 'calendar_event[title]=Paintball Fight!' \\      -F 'calendar_event[start_at]=2012-07-19T21:00:00Z' \\      -F 'calendar_event[end_at]=2012-07-19T22:00:00Z' \\      -H \"Authorization: Bearer <token>\" ``` |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendar_events_api_destroy

> calendar_events_api_destroy(id, cancel_reason, which)
Delete a calendar event

Delete an event from the calendar and return the deleted event

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**cancel_reason** | Option<**String**> | Reason for deleting/canceling the event. |  |
**which** | Option<**String**> | Valid if the event whose ID is in the URL is part of a series. Delete just the event whose ID is in in the URL, all events in the series, or the given event and all those following. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendar_events_api_get_course_timetable

> calendar_events_api_get_course_timetable(course_id)
Get course timetable

Returns the last timetable set by the {api:CalendarEventsApiController#set_course_timetable Set a course timetable} endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendar_events_api_index

> models::CalendarEvent calendar_events_api_index(r#type, start_date, end_date, undated, all_events, context_codes, excludes, includes, important_dates, blackout_date, page, per_page)
List calendar events

Retrieve the paginated list of calendar events or assignments for the current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | Option<**String**> | Defaults to \"event\" |  |
**start_date** | Option<**String**> | Only return events since the start_date (inclusive). Defaults to today. The value should be formatted as: yyyy-mm-dd or ISO 8601 YYYY-MM-DDTHH:MM:SSZ. |  |
**end_date** | Option<**String**> | Only return events before the end_date (inclusive). Defaults to start_date. The value should be formatted as: yyyy-mm-dd or ISO 8601 YYYY-MM-DDTHH:MM:SSZ. If end_date is the same as start_date, then only events on that day are returned. |  |
**undated** | Option<**bool**> | Defaults to false (dated events only). If true, only return undated events and ignore start_date and end_date. |  |
**all_events** | Option<**bool**> | Defaults to false (uses start_date, end_date, and undated criteria). If true, all events are returned, ignoring start_date, end_date, and undated criteria. |  |
**context_codes** | Option<[**Vec<String>**](String.md)> | [String] List of context codes of courses, groups, users, or accounts whose events you want to see. If not specified, defaults to the current user (i.e personal calendar, no course/group events). Limited to 10 context codes, additional ones are ignored. The format of this field is the context type, followed by an underscore, followed by the context id. For example: course_42 |  |
**excludes** | Option<[**Vec<String>**](String.md)> | [Array] Array of attributes to exclude. Possible values are \"description\", \"child_events\" and \"assignment\" |  |
**includes** | Option<[**Vec<String>**](String.md)> | [Array] Array of optional attributes to include. Possible values are \"web_conference\" and \"series_natural_language\" |  |
**important_dates** | Option<**bool**> | Defaults to false. If true, only events with important dates set to true will be returned. |  |
**blackout_date** | Option<**bool**> | Defaults to false. If true, only events with blackout date set to true will be returned. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::CalendarEvent**](CalendarEvent.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendar_events_api_reserve_other

> calendar_events_api_reserve_other(id, calendar_events_api_reserve_other_request)
Reserve a time slot

Reserves a particular time slot and return the new reservation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**calendar_events_api_reserve_other_request** | Option<[**CalendarEventsApiReserveOtherRequest**](CalendarEventsApiReserveOtherRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendar_events_api_reserve_other2

> calendar_events_api_reserve_other2(participant_id, id, calendar_events_api_reserve_other_request)
Reserve a time slot

Reserves a particular time slot and return the new reservation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**participant_id** | **String** | ID of the participant | [required] |
**id** | **String** | Scope response to id | [required] |
**calendar_events_api_reserve_other_request** | Option<[**CalendarEventsApiReserveOtherRequest**](CalendarEventsApiReserveOtherRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendar_events_api_save_enabled_account_calendars

> calendar_events_api_save_enabled_account_calendars(calendar_events_api_save_enabled_account_calendars_request)
Save enabled account calendars

Creates and updates the enabled_account_calendars and mark_feature_as_seen user preferences

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**calendar_events_api_save_enabled_account_calendars_request** | Option<[**CalendarEventsApiSaveEnabledAccountCalendarsRequest**](CalendarEventsApiSaveEnabledAccountCalendarsRequest.md)> | Request body parameters  **Example Request:** ``` curl 'https://<canvas>/api/v1/calendar_events/save_enabled_account_calendars' \\      -X POST \\      -F 'mark_feature_as_seen=true' \\      -F 'enabled_account_calendars[]=1' \\      -F 'enabled_account_calendars[]=2' \\      -H \"Authorization: Bearer <token>\" ``` |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendar_events_api_set_course_timetable

> calendar_events_api_set_course_timetable(course_id, calendar_events_api_set_course_timetable_request)
Set a course timetable

Creates and updates \"timetable\" events for a course. Can automaticaly generate a series of calendar events based on simple schedules (e.g. \"Monday and Wednesday at 2:00pm\" )  Existing timetable events for the course and course sections will be updated if they still are part of the timetable. Otherwise, they will be deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**calendar_events_api_set_course_timetable_request** | Option<[**CalendarEventsApiSetCourseTimetableRequest**](CalendarEventsApiSetCourseTimetableRequest.md)> | Request body parameters  **Example Request:** ``` curl 'https://<canvas>/api/v1/calendar_events/timetable' \\      -X POST \\      -F 'timetables[all][][weekdays]=Mon,Wed,Fri' \\      -F 'timetables[all][][start_time]=11:00 am' \\      -F 'timetables[all][][end_time]=11:50 am' \\      -F 'timetables[all][][location_name]=Room 237' \\      -H \"Authorization: Bearer <token>\" ``` |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendar_events_api_set_course_timetable_events

> calendar_events_api_set_course_timetable_events(course_id, calendar_events_api_set_course_timetable_events_request)
Create or update events directly for a course timetable

Creates and updates \"timetable\" events for a course or course section. Similar to {api:CalendarEventsApiController#set_course_timetable setting a course timetable}, but instead of generating a list of events based on a timetable schedule, this endpoint expects a complete list of events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**calendar_events_api_set_course_timetable_events_request** | Option<[**CalendarEventsApiSetCourseTimetableEventsRequest**](CalendarEventsApiSetCourseTimetableEventsRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendar_events_api_show

> String calendar_events_api_show(id)
Get a single calendar event or assignment

Returns detailed information about a specific calendar event or assignment.

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


## calendar_events_api_update

> calendar_events_api_update(id, calendar_events_api_update_request)
Update a calendar event

Update and return a calendar event

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**calendar_events_api_update_request** | Option<[**CalendarEventsApiUpdateRequest**](CalendarEventsApiUpdateRequest.md)> | Request body parameters  **Example Request:** ``` curl 'https://<canvas>/api/v1/calendar_events/234' \\      -X PUT \\      -F 'calendar_event[title]=Epic Paintball Fight!' \\      -H \"Authorization: Bearer <token>\" ``` |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calendar_events_api_user_index

> models::CalendarEvent calendar_events_api_user_index(user_id, r#type, start_date, end_date, undated, all_events, context_codes, excludes, submission_types, exclude_submission_types, includes, important_dates, blackout_date, page, per_page)
List calendar events for a user

Retrieve the paginated list of calendar events or assignments for the specified user. To view calendar events for a user other than yourself, you must either be an observer of that user or an administrator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Scope response to user_id | [required] |
**r#type** | Option<**String**> | Defaults to \"event\" |  |
**start_date** | Option<**String**> | Only return events since the start_date (inclusive). Defaults to today. The value should be formatted as: yyyy-mm-dd or ISO 8601 YYYY-MM-DDTHH:MM:SSZ. |  |
**end_date** | Option<**String**> | Only return events before the end_date (inclusive). Defaults to start_date. The value should be formatted as: yyyy-mm-dd or ISO 8601 YYYY-MM-DDTHH:MM:SSZ. If end_date is the same as start_date, then only events on that day are returned. |  |
**undated** | Option<**bool**> | Defaults to false (dated events only). If true, only return undated events and ignore start_date and end_date. |  |
**all_events** | Option<**bool**> | Defaults to false (uses start_date, end_date, and undated criteria). If true, all events are returned, ignoring start_date, end_date, and undated criteria. |  |
**context_codes** | Option<[**Vec<String>**](String.md)> | [String] List of context codes of courses, groups, users, or accounts whose events you want to see. If not specified, defaults to the current user (i.e personal calendar, no course/group events). Limited to 10 context codes, additional ones are ignored. The format of this field is the context type, followed by an underscore, followed by the context id. For example: course_42 |  |
**excludes** | Option<[**Vec<String>**](String.md)> | [Array] Array of attributes to exclude. Possible values are \"description\", \"child_events\" and \"assignment\" |  |
**submission_types** | Option<[**Vec<String>**](String.md)> | [Array] When type is \"assignment\", specifies the allowable submission types for returned assignments. Ignored if type is not \"assignment\" or if exclude_submission_types is provided. |  |
**exclude_submission_types** | Option<[**Vec<String>**](String.md)> | [Array] When type is \"assignment\", specifies the submission types to be excluded from the returned assignments. Ignored if type is not \"assignment\". |  |
**includes** | Option<[**Vec<String>**](String.md)> | [Array] Array of optional attributes to include. Possible values are \"web_conference\" and \"series_natural_language\" |  |
**important_dates** | Option<**bool**> | Defaults to false If true, only events with important dates set to true will be returned. |  |
**blackout_date** | Option<**bool**> | Defaults to false If true, only events with blackout date set to true will be returned. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::CalendarEvent**](CalendarEvent.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

