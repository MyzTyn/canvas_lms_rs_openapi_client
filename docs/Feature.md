# Feature

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**feature** | Option<**String**> | The symbolic name of the feature, used in FeatureFlags | [optional]
**display_name** | Option<**String**> | The user-visible name of the feature | [optional]
**applies_to** | Option<**AppliesTo**> | The type of object the feature applies to (RootAccount, Account, Course, or User):  * RootAccount features may only be controlled by flags on root accounts.  * Account features may be controlled by flags on accounts and their parent accounts.  * Course features may be controlled by flags on courses and their parent accounts.  * User features may be controlled by flags on users and site admin only. (enum: Course, RootAccount, Account, User) | [optional]
**feature_flag** | Option<[**models::FeatureFlag**](FeatureFlag.md)> |  | [optional]
**root_opt_in** | Option<**bool**> | If true, a feature that is 'allowed' globally will be 'off' by default in root accounts. Otherwise, root accounts inherit the global 'allowed' setting, which allows sub-accounts and courses to turn features on with no root account action. | [optional]
**beta** | Option<**bool**> | Whether the feature is a feature preview. If true, opting in includes ongoing updates outside the regular release schedule. | [optional]
**early_access_program** | Option<**bool**> | Indicates the feature is part of the Early Access Program. | [optional]
**autoexpand** | Option<**bool**> | Whether the details of the feature are autoexpanded on page load vs. the user clicking to expand. | [optional]
**release_notes_url** | Option<**String**> | A URL to the release notes describing the feature | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


