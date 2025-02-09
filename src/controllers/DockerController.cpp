#include "../../include/controllers/DockerController.h"
#include <drogon/HttpResponse.h>

using namespace drogon;

void DockerController::getContainers(const HttpRequestPtr &req,
                                     std::function<void(const HttpResponsePtr &)> &&callback)
{
    try
    {
        auto containers = dockerClient_.GetContainers();
        Json::Value jsonArray;
        for (const auto &container : containers)
        {
            jsonArray.append(container);
        }
        auto resp = HttpResponse::newHttpJsonResponse(jsonArray);
        callback(resp);
    }
    catch (const std::exception &e)
    {
        auto resp = HttpResponse::newHttpResponse();
        resp->setStatusCode(k500InternalServerError);
        resp->setBody(e.what());
        callback(resp);
    }
}

void DockerController::getPostgresContainers(const HttpRequestPtr &req,
                                             std::function<void(const HttpResponsePtr &)> &&callback)
{
    try
    {
        auto containers = dockerClient_.GetPostgresContainers();
        Json::Value jsonArray;
        for (const auto &container : containers)
        {
            jsonArray.append(container);
        }
        auto resp = HttpResponse::newHttpJsonResponse(jsonArray);
        callback(resp);
    }
    catch (const std::exception &e)
    {
        auto resp = HttpResponse::newHttpResponse();
        resp->setStatusCode(k500InternalServerError);
        resp->setBody(e.what());
        callback(resp);
    }
}

void DockerController::startContainer(const HttpRequestPtr &req,
                                      std::function<void(const HttpResponsePtr &)> &&callback,
                                      const std::string &container_id)
{
    try
    {
        bool success = dockerClient_.StartContainer(container_id);
        Json::Value response;
        response["success"] = success;
        auto resp = HttpResponse::newHttpJsonResponse(response);
        callback(resp);
    }
    catch (const std::exception &e)
    {
        auto resp = HttpResponse::newHttpResponse();
        resp->setStatusCode(k500InternalServerError);
        resp->setBody(e.what());
        callback(resp);
    }
}

void DockerController::stopContainer(const HttpRequestPtr &req,
                                     std::function<void(const HttpResponsePtr &)> &&callback,
                                     const std::string &container_id)
{
    try
    {
        bool success = dockerClient_.StopContainer(container_id);
        Json::Value response;
        response["success"] = success;
        auto resp = HttpResponse::newHttpJsonResponse(response);
        callback(resp);
    }
    catch (const std::exception &e)
    {
        auto resp = HttpResponse::newHttpResponse();
        resp->setStatusCode(k500InternalServerError);
        resp->setBody(e.what());
        callback(resp);
    }
}