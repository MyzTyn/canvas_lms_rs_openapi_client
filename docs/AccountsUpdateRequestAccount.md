# AccountsUpdateRequestAccount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | [String] Updates the account name | [optional]
**sis_account_id** | Option<**String**> | [String] Updates the account sis_account_id Must have manage_sis permission and must not be a root_account. | [optional]
**default_time_zone** | Option<**String**> | [String] The default time zone of the account. Allowed time zones are {http://www.iana.org/time-zones IANA time zones} or friendlier {http://api.rubyonrails.org/classes/ActiveSupport/TimeZone.html Ruby on Rails time zones}. | [optional]
**default_storage_quota_mb** | Option<**i32**> | [Integer] The default course storage quota to be used, if not otherwise specified. | [optional]
**default_user_storage_quota_mb** | Option<**i32**> | [Integer] The default user storage quota to be used, if not otherwise specified. | [optional]
**default_group_storage_quota_mb** | Option<**i32**> | [Integer] The default group storage quota to be used, if not otherwise specified. | [optional]
**course_template_id** | Option<**i32**> | [Integer] The ID of a course to be used as a template for all newly created courses. Empty means to inherit the setting from parent account, 0 means to not use a template even if a parent account has one set. The course must be marked as a template. | [optional]
**parent_account_id** | Option<**String**> | [Integer|String] The ID of a parent account to move the account to. The new parent account must be in the same root account as the original. The hierarchy of sub-accounts will be preserved in the new parent account. The caller must be an administrator in both the original parent account and the new parent account. | [optional]
**settings** | **String** | [lock_proficiency_calculation][value] [Boolean] [DEPRECATED] Restrict instructors from changing proficiency calculation method | 
**lock_outcome_proficiency** | Option<**String**> | [locked] [Boolean] [DEPRECATED] Lock this setting for sub-accounts and courses | [optional]
**lock_proficiency_calculation** | Option<**String**> | [locked] [Boolean] [DEPRECATED] Lock this setting for sub-accounts and courses | [optional]
**services** | Option<**String**> | [Hash] Give this a set of keys and boolean values to enable or disable services matching the keys | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


