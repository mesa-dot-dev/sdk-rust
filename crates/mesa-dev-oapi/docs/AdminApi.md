# \AdminApi

All URIs are relative to *https://depot.mesa.dev/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_by_org_api_keys_by_id**](AdminApi.md#delete_by_org_api_keys_by_id) | **DELETE** /{org}/api-keys/{id} | Revoke API key
[**get_by_org_api_keys**](AdminApi.md#get_by_org_api_keys) | **GET** /{org}/api-keys | List API keys
[**post_by_org_api_keys**](AdminApi.md#post_by_org_api_keys) | **POST** /{org}/api-keys | Create API key



## delete_by_org_api_keys_by_id

> models::DeleteByOrgApiKeysById200Response delete_by_org_api_keys_by_id(id, org)
Revoke API key

Revoke an API key by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**org** | **String** |  | [required] |

### Return type

[**models::DeleteByOrgApiKeysById200Response**](deleteByOrgApiKeysById_200_response.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_by_org_api_keys

> models::GetByOrgApiKeys200Response get_by_org_api_keys(org)
List API keys

List all API keys for the organization (key values are not returned)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** |  | [required] |

### Return type

[**models::GetByOrgApiKeys200Response**](getByOrgApiKeys_200_response.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_by_org_api_keys

> models::PostByOrgApiKeys201Response post_by_org_api_keys(org, post_by_org_api_keys_request)
Create API key

Create a new API key for programmatic access

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** |  | [required] |
**post_by_org_api_keys_request** | Option<[**PostByOrgApiKeysRequest**](PostByOrgApiKeysRequest.md)> |  |  |

### Return type

[**models::PostByOrgApiKeys201Response**](postByOrgApiKeys_201_response.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

