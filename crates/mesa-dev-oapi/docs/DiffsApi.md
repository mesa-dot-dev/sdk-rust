# \DiffsApi

All URIs are relative to *https://depot.mesa.dev/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_by_org_by_repo_diff**](DiffsApi.md#get_by_org_by_repo_diff) | **GET** /{org}/{repo}/diff | Get diff



## get_by_org_by_repo_diff

> models::GetByOrgByRepoDiff200Response get_by_org_by_repo_diff(org, repo, base, head)
Get diff

Retrieve the diff between two refs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** |  | [required] |
**repo** | **String** |  | [required] |
**base** | Option<**String**> |  | [required] |
**head** | Option<**String**> |  | [required] |

### Return type

[**models::GetByOrgByRepoDiff200Response**](getByOrgByRepoDiff_200_response.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

