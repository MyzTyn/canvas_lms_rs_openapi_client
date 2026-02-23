# \AppointmentGroupsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**appointment_groups_create**](AppointmentGroupsApi.md#appointment_groups_create) | **POST** /appointment_groups | Create an appointment group
[**appointment_groups_destroy**](AppointmentGroupsApi.md#appointment_groups_destroy) | **DELETE** /appointment_groups/{id} | Delete an appointment group
[**appointment_groups_groups**](AppointmentGroupsApi.md#appointment_groups_groups) | **GET** /appointment_groups/{id}/groups | List student group participants
[**appointment_groups_index**](AppointmentGroupsApi.md#appointment_groups_index) | **GET** /appointment_groups | List appointment groups
[**appointment_groups_next_appointment**](AppointmentGroupsApi.md#appointment_groups_next_appointment) | **GET** /appointment_groups/next_appointment | Get next appointment
[**appointment_groups_show**](AppointmentGroupsApi.md#appointment_groups_show) | **GET** /appointment_groups/{id} | Get a single appointment group
[**appointment_groups_update**](AppointmentGroupsApi.md#appointment_groups_update) | **PUT** /appointment_groups/{id} | Update an appointment group
[**appointment_groups_users**](AppointmentGroupsApi.md#appointment_groups_users) | **GET** /appointment_groups/{id}/users | List user participants



## appointment_groups_create

> appointment_groups_create(appointment_groups_create_request)
Create an appointment group

Create and return a new appointment group. If new_appointments are specified, the response will return a new_appointments array (same format as appointments array, see \"List appointment groups\" action)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**appointment_groups_create_request** | Option<[**AppointmentGroupsCreateRequest**](AppointmentGroupsCreateRequest.md)> | Request body parameters  **Example Request:** ``` curl 'https://<canvas>/api/v1/appointment_groups.json' \\      -X POST \\      -F 'appointment_group[context_codes][]=course_123' \\      -F 'appointment_group[sub_context_codes][]=course_section_234' \\      -F 'appointment_group[title]=Final Presentation' \\      -F 'appointment_group[participants_per_appointment]=1' \\      -F 'appointment_group[min_appointments_per_participant]=1' \\      -F 'appointment_group[max_appointments_per_participant]=1' \\      -F 'appointment_group[new_appointments][0][]=2012-07-19T21:00:00Z' \\      -F 'appointment_group[new_appointments][0][]=2012-07-19T22:00:00Z' \\      -F 'appointment_group[new_appointments][1][]=2012-07-19T22:00:00Z' \\      -F 'appointment_group[new_appointments][1][]=2012-07-19T23:00:00Z' \\      -H \"Authorization: Bearer <token>\" ``` |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## appointment_groups_destroy

> appointment_groups_destroy(id, cancel_reason)
Delete an appointment group

Delete an appointment group (and associated time slots and reservations) and return the deleted group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**cancel_reason** | Option<**String**> | Reason for deleting/canceling the appointment group. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## appointment_groups_groups

> appointment_groups_groups(id, registration_status_left_square_bracket_double_quote_all_double_quote_pipe_double_quote_registered_double_quote_pipe_double_quote_registered_double_quote_right_square_bracket, page, per_page)
List student group participants

A paginated list of student groups that are (or may be) participating in this appointment group. Refer to the Groups API for the response fields. Returns no results for appointment groups with the \"User\" participant_type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**registration_status_left_square_bracket_double_quote_all_double_quote_pipe_double_quote_registered_double_quote_pipe_double_quote_registered_double_quote_right_square_bracket** | Option<**String**> | Limits results to the a given participation status, defaults to \"all\" |  |
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


## appointment_groups_index

> appointment_groups_index(scope, context_codes, include_past_appointments, include, page, per_page)
List appointment groups

Retrieve the paginated list of appointment groups that can be reserved or managed by the current user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scope** | Option<**String**> | Defaults to \"reservable\" |  |
**context_codes** | Option<[**Vec<String>**](String.md)> | [String] Array of context codes used to limit returned results. |  |
**include_past_appointments** | Option<**bool**> | Defaults to false. If true, includes past appointment groups |  |
**include** | Option<[**Vec<String>**](String.md)> | [\"appointments\"|\"child_events\"|\"participant_count\"|\"reserved_times\"|\"all_context_codes\"] Array of additional information to include.  \"appointments\":: calendar event time slots for this appointment group \"child_events\":: reservations of those time slots \"participant_count\":: number of reservations \"reserved_times\":: the event id, start time and end time of reservations                    the current user has made) \"all_context_codes\":: all context codes associated with this appointment group |  |
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


## appointment_groups_next_appointment

> models::CalendarEvent appointment_groups_next_appointment(appointment_group_ids)
Get next appointment

Return the next appointment available to sign up for. The appointment is returned in a one-element array. If no future appointments are available, an empty array is returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**appointment_group_ids** | Option<[**Vec<String>**](String.md)> | [String] List of ids of appointment groups to search. |  |

### Return type

[**models::CalendarEvent**](CalendarEvent.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## appointment_groups_show

> appointment_groups_show(id, include)
Get a single appointment group

Returns information for a single appointment group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [\"child_events\"|\"appointments\"|\"all_context_codes\"] Array of additional information to include. See include[] argument of \"List appointment groups\" action.  \"child_events\":: reservations of time slots time slots \"appointments\":: will always be returned \"all_context_codes\":: all context codes associated with this appointment group |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## appointment_groups_update

> appointment_groups_update(id, appointment_groups_update_request)
Update an appointment group

Update and return an appointment group. If new_appointments are specified, the response will return a new_appointments array (same format as appointments array, see \"List appointment groups\" action).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**appointment_groups_update_request** | Option<[**AppointmentGroupsUpdateRequest**](AppointmentGroupsUpdateRequest.md)> | Request body parameters  **Example Request:** ``` curl 'https://<canvas>/api/v1/appointment_groups/543.json' \\      -X PUT \\      -F 'appointment_group[publish]=1' \\      -H \"Authorization: Bearer <token>\" ``` |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## appointment_groups_users

> appointment_groups_users(id, registration_status_left_square_bracket_double_quote_all_double_quote_pipe_double_quote_registered_double_quote_pipe_double_quote_registered_double_quote_right_square_bracket, page, per_page)
List user participants

A paginated list of users that are (or may be) participating in this appointment group.  Refer to the Users API for the response fields. Returns no results for appointment groups with the \"Group\" participant_type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**registration_status_left_square_bracket_double_quote_all_double_quote_pipe_double_quote_registered_double_quote_pipe_double_quote_registered_double_quote_right_square_bracket** | Option<**String**> | Limits results to the a given participation status, defaults to \"all\" |  |
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

