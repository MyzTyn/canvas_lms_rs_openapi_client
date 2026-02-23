# ServiceCredentialsUpdateRequestUser

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | [String] The full name of the user. This name will be used by teacher for grading. | [optional]
**short_name** | Option<**String**> | [String] User's name as it will be displayed in discussions, messages, and comments. | [optional]
**sortable_name** | Option<**String**> | [String] User's name as used to sort alphabetically in lists. | [optional]
**time_zone** | Option<**String**> | [String] The time zone for the user. Allowed time zones are {http://www.iana.org/time-zones IANA time zones} or friendlier {http://api.rubyonrails.org/classes/ActiveSupport/TimeZone.html Ruby on Rails time zones}. | [optional]
**email** | Option<**String**> | [String] The default email address of the user. | [optional]
**locale** | Option<**String**> | [String] The user's preferred language, from the list of languages Canvas supports. This is in RFC-5646 format. | [optional]
**avatar** | Option<**String**> | [state] [String, \"none\", \"submitted\", \"approved\", \"locked\", \"reported\", \"re_reported\"] To set the state of user's avatar. Only valid for account administrator. | [optional]
**title** | Option<**String**> | [String] Sets a title on the user profile. (See {api:ProfileController#settings Get user profile}.) Profiles must be enabled on the root account. | [optional]
**bio** | Option<**String**> | [String] Sets a bio on the user profile. (See {api:ProfileController#settings Get user profile}.) Profiles must be enabled on the root account. | [optional]
**pronunciation** | Option<**String**> | [String] Sets name pronunciation on the user profile. (See {api:ProfileController#settings Get user profile}.) Profiles and name pronunciation must be enabled on the root account. | [optional]
**pronouns** | Option<**String**> | [String] Sets pronouns on the user profile. Passing an empty string will empty the user's pronouns Only Available Pronouns set on the root account are allowed Adding and changing pronouns must be enabled on the root account. | [optional]
**event** | Option<**Event**> | [String, \"suspend\"|\"unsuspend\"] Suspends or unsuspends all logins for this user that the calling user has permission to (enum: suspend, unsuspend) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


