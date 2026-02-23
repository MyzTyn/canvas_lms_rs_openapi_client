# CalendarEventsApiUpdateRequestCalendarEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**context_code** | Option<**String**> | [Optional, String] Context code of the course, group, user, or account to move this event to. Scheduler appointments and events with section-specific times cannot be moved between calendars. | [optional]
**title** | Option<**String**> | [String] Short title for the calendar event. | [optional]
**description** | Option<**String**> | [String] Longer HTML description of the event. | [optional]
**start_at** | Option<**String**> | [DateTime] Start date/time of the event. | [optional]
**end_at** | Option<**String**> | [DateTime] End date/time of the event. | [optional]
**location_name** | Option<**String**> | [String] Location name of the event. | [optional]
**location_address** | Option<**String**> | [String] Location address | [optional]
**time_zone_edited** | Option<**String**> | [String] Time zone of the user editing the event. Allowed time zones are {http://www.iana.org/time-zones IANA time zones} or friendlier {http://api.rubyonrails.org/classes/ActiveSupport/TimeZone.html Ruby on Rails time zones}. | [optional]
**all_day** | Option<**bool**> | [Boolean] When true event is considered to span the whole day and times are ignored. | [optional]
**child_event_data** | Option<**String**> | [X][context_code] [String] Context code(s) corresponding to the section-level start and end time(s). | [optional]
**rrule** | Option<**String**> | [Optional, String] Valid if the event whose ID is in the URL is part of a series. This defines the shape of the recurring event series after it's updated. Its value is the iCalendar RRULE. Unending series are not supported. | [optional]
**blackout_date** | Option<**bool**> | [Boolean] If the blackout_date is true, this event represents a holiday or some other special day that does not count in course pacing. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


