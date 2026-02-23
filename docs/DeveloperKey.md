# DeveloperKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The Canvas ID of the DeveloperKey object | [optional]
**name** | Option<**String**> | The display name | [optional]
**created_at** | Option<**String**> | Timestamp of the key's creation | [optional]
**updated_at** | Option<**String**> | Timestamp of the key's last update | [optional]
**workflow_state** | Option<**String**> | The state of the key | [optional]
**is_lti_key** | Option<**bool**> | True if key represents an LTI 1.3 Registration. False for Canvas API keys | [optional]
**email** | Option<**String**> | Contact email configured for key | [optional]
**icon_url** | Option<**String**> | URL for a small icon to display in key list | [optional]
**notes** | Option<**String**> | User-provided notes about key | [optional]
**vendor_code** | Option<**String**> | User-specified code representing the vendor that uses the key | [optional]
**account_name** | Option<**String**> | The name of the account that owns the key | [optional]
**visible** | Option<**bool**> | True for all keys except Site Admin-level keys, which default to false. Controls visibility in the Inherited tab. | [optional]
**scopes** | Option<**Vec<String>**> | List of API endpoints key is allowed to access (API keys), or LTI 1.3 scopes (LTI keys) | [optional]
**redirect_uri** | Option<**String**> | Deprecated in favor of redirect_uris. Do not use. | [optional]
**redirect_uris** | Option<**Vec<String>**> | List of URLs used during OAuth2 flow to validate given redirect URI (API keys), or to redirect to after login (LTI keys) | [optional]
**access_token_count** | Option<**i32**> | (API keys only) The number of active access tokens associated with the key | [optional]
**last_used_at** | Option<**String**> | (API keys only) The last time an access token for this key was used in an API request | [optional]
**test_cluster_only** | Option<**bool**> | (API keys only) If true, key is only usable in non-production environments (test, beta). Avoids problems with beta refresh. | [optional]
**allow_includes** | Option<**bool**> | (API keys only) If true, allows `includes` parameters in API requests that match the scopes of this key | [optional]
**require_scopes** | Option<**bool**> | (API keys only) If true, then token requests with this key must include scopes | [optional]
**client_credentials_audience** | Option<**String**> | (API keys only) Used in OAuth2 client credentials flow to specify the audience for the access token | [optional]
**api_key** | Option<**String**> | (API keys only) The client secret used in the OAuth authorization_code flow. | [optional]
**tool_configuration** | Option<[**models::LtiToolConfiguration**](LtiToolConfiguration.md)> |  | [optional]
**public_jwk** | Option<**serde_json::Value**> | (LTI keys only) The tool's public JWK in JSON format. Discouraged in favor of a url hosting a JWK set. | [optional]
**public_jwk_url** | Option<**String**> | (LTI keys only) The tool-hosted URL containing its public JWK keyset. Canvas may cache JWKs up to 5 minutes. | [optional]
**lti_registration** | Option<**serde_json::Value**> | (LTI keys only) The LTI IMS Registration object for this key, if key was created via Dynamic Registration. | [optional]
**is_lti_registration** | Option<**bool**> | (LTI keys only) Returns true if key was created via Dynamic Registration. | [optional]
**user_name** | Option<**String**> | Unused. | [optional]
**user_id** | Option<**String**> | Unused. | [optional]
**unified_tool_id** | Option<**String**> | Correlates an API key to a product configuration. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


