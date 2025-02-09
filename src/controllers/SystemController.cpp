#include "../../include/controllers/SystemController.h"
#include <drogon/HttpResponse.h>

using namespace drogon;

void SystemController::uptime(const HttpRequestPtr &req,
                              std::function<void(const HttpResponsePtr &)> &&callback)
{
    Json::Value response;
    response["uptime"] = static_cast<Json::UInt64>(monitor_.GetUptime());
    auto resp = HttpResponse::newHttpJsonResponse(response);
    callback(resp);
}

void SystemController::cpuLoad(const HttpRequestPtr &req,
                               std::function<void(const HttpResponsePtr &)> &&callback)
{
    Json::Value response;
    response["cpu_load"] = monitor_.GetCpuLoad();
    auto resp = HttpResponse::newHttpJsonResponse(response);
    callback(resp);
}

void SystemController::memory(const HttpRequestPtr &req,
                              std::function<void(const HttpResponsePtr &)> &&callback)
{
    Json::Value response;
    response["used_memory"] = static_cast<Json::UInt64>(monitor_.GetUsedMemory());
    response["total_memory"] = static_cast<Json::UInt64>(monitor_.GetTotalMemory());
    auto resp = HttpResponse::newHttpJsonResponse(response);
    callback(resp);
}

void SystemController::diskUsage(const HttpRequestPtr &req,
                                 std::function<void(const HttpResponsePtr &)> &&callback)
{
    Json::Value response;
    response["disk_usage"] = monitor_.GetDiskUsage();
    auto resp = HttpResponse::newHttpJsonResponse(response);
    callback(resp);
}