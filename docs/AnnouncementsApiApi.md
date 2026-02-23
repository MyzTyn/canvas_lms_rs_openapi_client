# \AnnouncementsApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**announcements_api_index**](AnnouncementsApiApi.md#announcements_api_index) | **GET** /announcements | List announcements



## announcements_api_index

> models::DiscussionTopic announcements_api_index(context_codes, start_date, end_date, available_after, active_only, latest_only, include, page, per_page)
List announcements

Returns the paginated list of announcements for the given courses and date range.  Note that a +context_code+ field is added to the responses so you can tell which course each announcement belongs to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_codes** | [**Vec<String>**](String.md) | [Required] List of context_codes to retrieve announcements for (for example, +course_123+). Only courses are presently supported. The call will fail unless the caller has View Announcements permission in all listed courses. | [required] |
**start_date** | Option<**String**> | Only return announcements posted since the start_date (inclusive). Defaults to 14 days ago. The value should be formatted as: yyyy-mm-dd or ISO 8601 YYYY-MM-DDTHH:MM:SSZ. |  |
**end_date** | Option<**String**> | Only return announcements posted before the end_date (inclusive). Defaults to 28 days from start_date. The value should be formatted as: yyyy-mm-dd or ISO 8601 YYYY-MM-DDTHH:MM:SSZ. Announcements scheduled for future posting will only be returned to course administrators. |  |
**available_after** | Option<**String**> | Only return announcements having locked_at nil or after available_after (exclusive). The value should be formatted as: yyyy-mm-dd or ISO 8601 YYYY-MM-DDTHH:MM:SSZ. Effective only for students (who don't have moderate forum right). |  |
**active_only** | Option<**bool**> | Only return active announcements that have been published. Applies only to requesting users that have permission to view unpublished items. Defaults to false for users with access to view unpublished items, otherwise true and unmodifiable. |  |
**latest_only** | Option<**bool**> | Only return the latest announcement for each associated context. The response will include at most one announcement for each specified context in the context_codes[] parameter. Defaults to false. |  |
**include** | Option<[**Vec<String>**](String.md)> | Optional list of resources to include with the response. May include a string of the name of the resource. Possible values are: \"sections\", \"sections_user_count\" if \"sections\" is passed, includes the course sections that are associated with the topic, if the topic is specific to certain sections of the course. If \"sections_user_count\" is passed, then:   (a) If sections were asked for *and* the topic is specific to certain       course sections sections, includes the number of users in each       section. (as part of the section json asked for above)   (b) Else, includes at the root level the total number of users in the       topic's context (group or course) that the topic applies to. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::DiscussionTopic**](DiscussionTopic.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

