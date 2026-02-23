# ServiceCredentialsCreateSelfRegisteredUserRequestUser

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | [Required, String] The full name of the user. This name will be used by teacher for grading. | 
**short_name** | Option<**String**> | [String] User's name as it will be displayed in discussions, messages, and comments. | [optional]
**sortable_name** | Option<**String**> | [String] User's name as used to sort alphabetically in lists. | [optional]
**time_zone** | Option<**String**> | [String] The time zone for the user. Allowed time zones are {http://www.iana.org/time-zones IANA time zones} or friendlier {http://api.rubyonrails.org/classes/ActiveSupport/TimeZone.html Ruby on Rails time zones}. | [optional]
**locale** | Option<**String**> | [String] The user's preferred language, from the list of languages Canvas supports. This is in RFC-5646 format. | [optional]
**terms_of_use** | **bool** | [Required, Boolean] Whether the user accepts the terms of use. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


