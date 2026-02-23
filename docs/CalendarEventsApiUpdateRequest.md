# CalendarEventsApiUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**which** | Option<**Which**> | Valid if the event whose ID is in the URL is part of a series. Update just the event whose ID is in in the URL, all events in the series, or the given event and all those following. Some updates may create a new series. For example, changing the start time of this and all following events from the middle of a series. (enum: one, all, following) | [optional]
**calendar_event** | Option<[**models::CalendarEventsApiUpdateRequestCalendarEvent**](CalendarEventsApiUpdateRequestCalendarEvent.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


