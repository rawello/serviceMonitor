#pragma once
#include <drogon/HttpController.h>
#include "../system_monitor.h"

class SystemController : public drogon::HttpController<SystemController>
{
public:
    METHOD_LIST_BEGIN
    ADD_METHOD_TO(SystemController::uptime,
                  "/system/uptime",
                  drogon::Get);
    ADD_METHOD_TO(SystemController::cpuLoad,
                  "/system/cpu-load",
                  drogon::Get);
    ADD_METHOD_TO(SystemController::memory,
                  "/system/memory",
                  drogon::Get);
    ADD_METHOD_TO(SystemController::diskUsage,
                  "/system/disk-usage",
                  drogon::Get);
    METHOD_LIST_END

    void uptime(const drogon::HttpRequestPtr &req,
                std::function<void(const drogon::HttpResponsePtr &)> &&callback);

    void cpuLoad(const drogon::HttpRequestPtr &req,
                 std::function<void(const drogon::HttpResponsePtr &)> &&callback);

    void memory(const drogon::HttpRequestPtr &req,
                std::function<void(const drogon::HttpResponsePtr &)> &&callback);

    void diskUsage(const drogon::HttpRequestPtr &req,
                   std::function<void(const drogon::HttpResponsePtr &)> &&callback);

private:
    SystemMonitor monitor_;
};