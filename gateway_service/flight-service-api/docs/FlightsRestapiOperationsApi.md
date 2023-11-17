# \FlightsRestapiOperationsApi

All URIs are relative to *http://localhost:8060*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_flight**](FlightsRestapiOperationsApi.md#get_flight) | **GET** /api/v1/flights/{id} | Get Flight inforation by ID



## get_flight

> crate::models::FlightResponse get_flight(id)
Get Flight inforation by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**crate::models::FlightResponse**](FlightResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

