# \ContentApi

All URIs are relative to *https://depot.mesa.dev/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_by_org_by_repo_content**](ContentApi.md#get_by_org_by_repo_content) | **GET** /{org}/{repo}/content | Get content



## get_by_org_by_repo_content

> models::GetByOrgByRepoContent200Response get_by_org_by_repo_content(org, repo, r#ref, path, depth)
Get content

Get file content or directory listing at a path. Use Accept: application/json for the JSON union response, or Accept: application/octet-stream for raw file bytes. Directory + octet-stream requests return 406 Not Acceptable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** |  | [required] |
**repo** | **String** |  | [required] |
**r#ref** | Option<**String**> |  |  |
**path** | Option<**String**> |  |  |
**depth** | Option<**u64**> |  |  |

### Return type

[**models::GetByOrgByRepoContent200Response**](getByOrgByRepoContent_200_response.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

