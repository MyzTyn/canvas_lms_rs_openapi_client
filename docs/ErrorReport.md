# ErrorReport

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**subject** | Option<**String**> | The users problem summary, like an email subject line | [optional]
**comments** | Option<**String**> | long form documentation of what was witnessed | [optional]
**user_perceived_severity** | Option<**String**> | categorization of how bad the user thinks the problem is.  Should be one of [just_a_comment, not_urgent, workaround_possible, blocks_what_i_need_to_do, extreme_critical_emergency]. | [optional]
**email** | Option<**String**> | the email address of the reporting user | [optional]
**url** | Option<**String**> | URL of the page on which the error was reported | [optional]
**context_asset_string** | Option<**String**> | string describing the asset being interacted with at the time of error.  Formatted '[type]_[id]' | [optional]
**user_roles** | Option<**String**> | comma seperated list of roles the reporting user holds.  Can be one [student], or many [teacher,admin] | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


