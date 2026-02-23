# CalendarEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The ID of the calendar event | [optional]
**title** | Option<**String**> | The title of the calendar event | [optional]
**start_at** | Option<**String**> | The start timestamp of the event | [optional]
**end_at** | Option<**String**> | The end timestamp of the event | [optional]
**description** | Option<**String**> | The HTML description of the event | [optional]
**location_name** | Option<**String**> | The location name of the event | [optional]
**location_address** | Option<**String**> | The address where the event is taking place | [optional]
**context_code** | Option<**String**> | the context code of the calendar this event belongs to (course, group, user, or account) | [optional]
**effective_context_code** | Option<**String**> | if specified, it indicates which calendar this event should be displayed on. for example, a section-level event would have the course's context code here, while the section's context code would be returned above) | [optional]
**context_name** | Option<**String**> | the context name of the calendar this event belongs to (course, user or group) | [optional]
**all_context_codes** | Option<**String**> | a comma-separated list of all calendar contexts this event is part of | [optional]
**workflow_state** | Option<**String**> | Current state of the event ('active', 'locked' or 'deleted') 'locked' indicates that start_at/end_at cannot be changed (though the event could be deleted). Normally only reservations or time slots with reservations are locked (see the Appointment Groups API) | [optional]
**hidden** | Option<**bool**> | Whether this event should be displayed on the calendar. Only true for course-level events with section-level child events. | [optional]
**parent_event_id** | Option<**i32**> | Normally null. If this is a reservation (see the Appointment Groups API), the id will indicate the time slot it is for. If this is a section-level event, this will be the course-level parent event. | [optional]
**child_events_count** | Option<**i32**> | The number of child_events. See child_events (and parent_event_id) | [optional]
**child_events** | Option<**Vec<i32>**> | Included by default, but may be excluded (see include[] option). If this is a time slot (see the Appointment Groups API) this will be a list of any reservations. If this is a course-level event, this will be a list of section-level events (if any) | [optional]
**url** | Option<**String**> | URL for this calendar event (to update, delete, etc.) | [optional]
**html_url** | Option<**String**> | URL for a user to view this event | [optional]
**all_day_date** | Option<**String**> | The date of this event | [optional]
**all_day** | Option<**bool**> | Boolean indicating whether this is an all-day event (midnight to midnight) | [optional]
**created_at** | Option<**String**> | When the calendar event was created | [optional]
**updated_at** | Option<**String**> | When the calendar event was last updated | [optional]
**appointment_group_id** | Option<**i32**> | Various Appointment-Group-related fields.These fields are only pertinent to time slots (appointments) and reservations of those time slots. See the Appointment Groups API. The id of the appointment group | [optional]
**appointment_group_url** | Option<**String**> | The API URL of the appointment group | [optional]
**own_reservation** | Option<**bool**> | If the event is a reservation, this a boolean indicating whether it is the current user's reservation, or someone else's | [optional]
**reserve_url** | Option<**String**> | If the event is a time slot, the API URL for reserving it | [optional]
**reserved** | Option<**bool**> | If the event is a time slot, a boolean indicating whether the user has already made a reservation for it | [optional]
**participant_type** | Option<**String**> | The type of participant to sign up for a slot: 'User' or 'Group' | [optional]
**participants_per_appointment** | Option<**i32**> | If the event is a time slot, this is the participant limit | [optional]
**available_slots** | Option<**i32**> | If the event is a time slot and it has a participant limit, an integer indicating how many slots are available | [optional]
**user** | Option<**String**> | If the event is a user-level reservation, this will contain the user participant JSON (refer to the Users API). | [optional]
**group** | Option<**String**> | If the event is a group-level reservation, this will contain the group participant JSON (refer to the Groups API). | [optional]
**important_dates** | Option<**bool**> | Boolean indicating whether this has important dates. | [optional]
**series_uuid** | Option<**uuid::Uuid**> | Identifies the recurring event series this event may belong to. | [optional]
**rrule** | Option<**String**> | An iCalendar RRULE for defining how events in a recurring event series repeat. | [optional]
**series_head** | Option<**bool**> | Boolean indicating if is the first event in the series of recurring events. | [optional]
**series_natural_language** | Option<**String**> | A natural language expression of how events occur in the series. | [optional]
**blackout_date** | Option<**bool**> | Boolean indicating whether this has blackout date. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


