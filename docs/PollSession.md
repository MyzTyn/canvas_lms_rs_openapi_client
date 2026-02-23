# PollSession

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | The unique identifier for the poll session. | 
**poll_id** | **i32** | The id of the Poll this poll session is associated with | 
**course_id** | **i32** | The id of the Course this poll session is associated with | 
**course_section_id** | Option<**i32**> | The id of the Course Section this poll session is associated with | [optional]
**is_published** | Option<**bool**> | Specifies whether or not this poll session has been published for students to participate in. | [optional]
**has_public_results** | Option<**bool**> | Specifies whether the results are viewable by students. | [optional]
**created_at** | Option<**String**> | The time at which the poll session was created. | [optional]
**results** | Option<**serde_json::Value**> | The results of the submissions of the poll. Each key is the poll choice id, and the value is the count of submissions. | [optional]
**poll_submissions** | Option<[**models::PollSubmission**](PollSubmission.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


