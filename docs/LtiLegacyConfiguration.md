# LtiLegacyConfiguration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | Option<**String**> | The display name of the tool | [optional]
**description** | Option<**String**> | The description of the tool | [optional]
**custom_fields** | Option<**serde_json::Value**> | A key-value listing of all custom fields the tool has requested | [optional]
**target_link_uri** | Option<**String**> | The default launch URL for the tool. Overridable by placements. | [optional]
**oidc_initiation_url** | Option<**String**> | 1.3 specific. URL used for initial login request | [optional]
**oidc_initiation_urls** | Option<**serde_json::Value**> | 1.3 specific. Region-specific login URLs for data protection compliance | [optional]
**public_jwk** | Option<**serde_json::Value**> | 1.3 specific. The tool's public JWK in JSON format. Discouraged in favor of a url hosting a JWK set. | [optional]
**public_jwk_url** | Option<**String**> | 1.3 specific. The tool-hosted URL containing its public JWK keyset. Canvas may cache JWKs up to 5 minutes. | [optional]
**scopes** | Option<**Vec<String>**> | 1.3 specific. List of LTI scopes requested by the tool | [optional]
**extensions** | Option<[**Vec<models::LtiLegacyConfigurationExtensionsInner>**](LtiLegacyConfigurationExtensionsInner.md)> | Array of extensions for the tool | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


