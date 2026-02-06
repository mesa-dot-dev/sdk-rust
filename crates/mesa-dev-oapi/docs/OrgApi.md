# \OrgApi

All URIs are relative to *https://depot.mesa.dev/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_by_org**](OrgApi.md#get_by_org) | **GET** /{org} | Get organization



## get_by_org

> models::GetByOrg200Response get_by_org(org)
Get organization

Get organization metadata and repository counts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** |  | [required] |

### Return type

[**models::GetByOrg200Response**](getByOrg_200_response.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

