# CalendarEventsApiSetCourseTimetableEventsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**course_section_id** | Option<**String**> | Events will be created for the course section specified by course_section_id. If not present, events will be created for the entire course. | [optional]
**events** | Option<**Vec<String>**> | [Array] An array of event objects to use. | [optional]
**events_left_square_bracket_right_square_bracket_left_square_bracket_start_at_right_square_bracket** | Option<**String**> | [DateTime] Start time for the event | [optional]
**events_left_square_bracket_right_square_bracket_left_square_bracket_end_at_right_square_bracket** | Option<**String**> | [DateTime] End time for the event | [optional]
**events_left_square_bracket_right_square_bracket_left_square_bracket_location_name_right_square_bracket** | Option<**String**> | [Optional, String] Location name for the event | [optional]
**events_left_square_bracket_right_square_bracket_left_square_bracket_code_right_square_bracket** | Option<**String**> | [Optional, String] A unique identifier that can be used to update the event at a later time If one is not specified, an identifier will be generated based on the start and end times | [optional]
**events_left_square_bracket_right_square_bracket_left_square_bracket_title_right_square_bracket** | Option<**String**> | [Optional, String] Title for the meeting. If not present, will default to the associated course's name | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


