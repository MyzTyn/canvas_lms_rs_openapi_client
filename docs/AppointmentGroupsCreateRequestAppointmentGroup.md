# AppointmentGroupsCreateRequestAppointmentGroup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**context_codes** | **String** | [] [Required, String] Array of context codes (courses, e.g. course_1) this group should be linked to (1 or more). Users in the course(s) with appropriate permissions will be able to sign up for this appointment group. | 
**sub_context_codes** | Option<**String**> | [] [String] Array of sub context codes (course sections or a single group category) this group should be linked to. Used to limit the appointment group to particular sections. If a group category is specified, students will sign up in groups and the participant_type will be \"Group\" instead of \"User\". | [optional]
**title** | **String** | [Required, String] Short title for the appointment group. | 
**description** | Option<**String**> | [String] Longer text description of the appointment group. | [optional]
**location_name** | Option<**String**> | [String] Location name of the appointment group. | [optional]
**location_address** | Option<**String**> | [String] Location address. | [optional]
**publish** | Option<**bool**> | [Boolean] Indicates whether this appointment group should be published (i.e. made available for signup). Once published, an appointment group cannot be unpublished. Defaults to false. | [optional]
**participants_per_appointment** | Option<**i32**> | [Integer] Maximum number of participants that may register for each time slot. Defaults to null (no limit). | [optional]
**min_appointments_per_participant** | Option<**i32**> | [Integer] Minimum number of time slots a user must register for. If not set, users do not need to sign up for any time slots. | [optional]
**max_appointments_per_participant** | Option<**i32**> | [Integer] Maximum number of time slots a user may register for. | [optional]
**new_appointments** | Option<**String**> | [X][] Nested array of start time/end time pairs indicating time slots for this appointment group. Refer to the example request. | [optional]
**participant_visibility** | Option<**String**> | [\"private\"|\"protected\"] \"private\":: participants cannot see who has signed up for a particular             time slot \"protected\":: participants can see who has signed up.  Defaults to               \"private\". | [optional]
**allow_observer_signup** | Option<**bool**> | [Boolean] Whether observer users can sign-up for an appointment. Defaults to false. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


