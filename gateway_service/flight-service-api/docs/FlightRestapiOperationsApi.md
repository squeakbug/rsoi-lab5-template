# \FlightRestapiOperationsApi

All URIs are relative to *http://localhost:8060*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_flights**](FlightRestapiOperationsApi.md#list_flights) | **GET** /api/v1/flights | Получить список всех перелетов



## list_flights

> crate::models::PaginationResponse list_flights(page, size, flight_number)
Получить список всех перелетов

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |
**size** | Option<**i32**> |  |  |
**flight_number** | Option<**String**> |  |  |

### Return type

[**crate::models::PaginationResponse**](PaginationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

