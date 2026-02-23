# NamesAndRoleMembership

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | Option<**String**> | Membership state | [optional]
**name** | Option<**String**> | Member's full name. Only included if tool privacy level is `public` or `name_only`. | [optional]
**picture** | Option<**String**> | URL to the member's avatar. Only included if tool privacy level is `public`. | [optional]
**given_name** | Option<**String**> | Member's 'first' name. Only included if tool privacy level is `public` or `name_only`. | [optional]
**family_name** | Option<**String**> | Member's 'last' name. Only included if tool privacy level is `public` or `name_only`. | [optional]
**email** | Option<**String**> | Member's email address. Only included if tool privacy level is `public` or `email_only`. | [optional]
**lis_person_sourcedid** | Option<**String**> | Member's primary SIS identifier. Only included if tool privacy level is `public` or `name_only`. | [optional]
**user_id** | Option<**String**> | Member's unique LTI identifier. | [optional]
**roles** | Option<**Vec<Roles>**> | Member's roles in the current Context, expressed as LTI/LIS URNs. (enum: http://purl.imsglobal.org/vocab/lis/v2/membership/Instructor#TeachingAssistant, http://purl.imsglobal.org/vocab/lis/v2/membership#Learner, http://purl.imsglobal.org/vocab/lis/v2/membership#Instructor, http://purl.imsglobal.org/vocab/lis/v2/membership#ContentDeveloper, http://purl.imsglobal.org/vocab/lis/v2/membership#Mentor, http://purl.imsglobal.org/vocab/lis/v2/membership#Member, http://purl.imsglobal.org/vocab/lis/v2/membership#Manager) | [optional]
**message** | Option<[**Vec<models::NamesAndRoleMessage>**](NamesAndRoleMessage.md)> | Only present when the request specifies a `rlid` query parameter. Contains additional attributes which would appear in the LTI launch message were this member to click the link referenced by the `rlid` query parameter | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


