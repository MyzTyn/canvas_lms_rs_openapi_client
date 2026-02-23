# CalendarEventsApiCreateRequestCalendarEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**context_code** | **String** | [Required, String] Context code of the course, group, user, or account whose calendar this event should be added to. | 
**title** | Option<**String**> | [String] Short title for the calendar event. | [optional]
**description** | Option<**String**> | [String] Longer HTML description of the event. | [optional]
**start_at** | Option<**String**> | [DateTime] Start date/time of the event. | [optional]
**end_at** | Option<**String**> | [DateTime] End date/time of the event. | [optional]
**location_name** | Option<**String**> | [String] Location name of the event. | [optional]
**location_address** | Option<**String**> | [String] Location address | [optional]
**time_zone_edited** | Option<**String**> | [String] Time zone of the user editing the event. Allowed time zones are {http://www.iana.org/time-zones IANA time zones} or friendlier {http://api.rubyonrails.org/classes/ActiveSupport/TimeZone.html Ruby on Rails time zones}. | [optional]
**all_day** | Option<**bool**> | [Boolean] When true event is considered to span the whole day and times are ignored. | [optional]
**child_event_data** | Option<**String**> | [X][context_code] [String] Context code(s) corresponding to the section-level start and end time(s). | [optional]
**duplicate** | Option<**String**> | [append_iterator] [Boolean] Defaults to false.  If set to `true`, an increasing counter number will be appended to the event title when the event is duplicated.  (e.g. Event 1, Event 2, Event 3, etc) | [optional]
**rrule** | Option<**String**> | [string] The recurrence rule to create a series of recurring events. Its value is the {https://icalendar.org/iCalendar-RFC-5545/3-8-5-3-recurrence-rule.html iCalendar RRULE} defining how the event repeats. Unending series not supported. | [optional]
**blackout_date** | Option<**bool**> | [Boolean] If the blackout_date is true, this event represents a holiday or some other special day that does not count in course pacing. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


