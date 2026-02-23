# AppointmentGroup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The ID of the appointment group | [optional]
**title** | Option<**String**> | The title of the appointment group | [optional]
**start_at** | Option<**String**> | The start of the first time slot in the appointment group | [optional]
**end_at** | Option<**String**> | The end of the last time slot in the appointment group | [optional]
**description** | Option<**String**> | The text description of the appointment group | [optional]
**location_name** | Option<**String**> | The location name of the appointment group | [optional]
**location_address** | Option<**String**> | The address of the appointment group's location | [optional]
**participant_count** | Option<**i32**> | The number of participant who have reserved slots (see include[] argument) | [optional]
**reserved_times** | Option<[**Vec<models::Appointment>**](Appointment.md)> | The start and end times of slots reserved by the current user as well as the id of the calendar event for the reservation (see include[] argument) | [optional]
**allow_observer_signup** | Option<**bool**> | Boolean indicating whether observer users should be able to sign-up for an appointment | [optional]
**context_codes** | Option<**Vec<String>**> | The context codes (i.e. courses) this appointment group belongs to. Only people in these courses will be eligible to sign up. | [optional]
**sub_context_codes** | Option<**Vec<i32>**> | The sub-context codes (i.e. course sections and group categories) this appointment group is restricted to | [optional]
**workflow_state** | Option<**WorkflowState**> | Current state of the appointment group ('pending', 'active' or 'deleted'). 'pending' indicates that it has not been published yet and is invisible to participants. (enum: pending, active, deleted) | [optional]
**requiring_action** | Option<**bool**> | Boolean indicating whether the current user needs to sign up for this appointment group (i.e. it's reservable and the min_appointments_per_participant limit has not been met by this user). | [optional]
**appointments_count** | Option<**i32**> | Number of time slots in this appointment group | [optional]
**appointments** | Option<[**Vec<models::CalendarEvent>**](CalendarEvent.md)> | Calendar Events representing the time slots (see include[] argument) Refer to the Calendar Events API for more information | [optional]
**new_appointments** | Option<[**Vec<models::CalendarEvent>**](CalendarEvent.md)> | Newly created time slots (same format as appointments above). Only returned in Create/Update responses where new time slots have been added | [optional]
**max_appointments_per_participant** | Option<**i32**> | Maximum number of time slots a user may register for, or null if no limit | [optional]
**min_appointments_per_participant** | Option<**i32**> | Minimum number of time slots a user must register for. If not set, users do not need to sign up for any time slots | [optional]
**participants_per_appointment** | Option<**i32**> | Maximum number of participants that may register for each time slot, or null if no limit | [optional]
**participant_visibility** | Option<**ParticipantVisibility**> | 'private' means participants cannot see who has signed up for a particular time slot, 'protected' means that they can (enum: private, protected) | [optional]
**participant_type** | Option<**ParticipantType**> | Indicates how participants sign up for the appointment group, either as individuals ('User') or in student groups ('Group'). Related to sub_context_codes (i.e. 'Group' signups always have a single group category) (enum: User, Group) | [optional]
**url** | Option<**String**> | URL for this appointment group (to update, delete, etc.) | [optional]
**html_url** | Option<**String**> | URL for a user to view this appointment group | [optional]
**created_at** | Option<**String**> | When the appointment group was created | [optional]
**updated_at** | Option<**String**> | When the appointment group was last updated | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


