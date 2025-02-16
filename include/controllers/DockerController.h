#pragma once
#include <drogon/HttpController.h>
#include "../docker_client.h"

class DockerController : public drogon::HttpController<DockerController> {
 public:
  METHOD_LIST_BEGIN
  ADD_METHOD_TO(DockerController::getContainers,
                "/docker/containers",
                drogon::Get);
  ADD_METHOD_TO(DockerController::startContainer,
                "/docker/{anystring}/start",
                drogon::Post);
  ADD_METHOD_TO(DockerController::stopContainer,
                "/docker/{anystring}/stop",
                drogon::Post);
  ADD_METHOD_TO(DockerController::restartContainer,
                "/docker/{anystring}/restart",
                drogon::Post);
  ADD_METHOD_TO(DockerController::getContainerLogs,
                "/docker/{anystring}/logs",
                drogon::Get);
  METHOD_LIST_END

  void getContainers(
      const drogon::HttpRequestPtr& req,
      std::function<void(const drogon::HttpResponsePtr&)>&& callback);

  void startContainer(
      const drogon::HttpRequestPtr& req,
      std::function<void(const drogon::HttpResponsePtr&)>&& callback,
      const std::string& container_id);

  void stopContainer(
      const drogon::HttpRequestPtr& req,
      std::function<void(const drogon::HttpResponsePtr&)>&& callback,
      const std::string& container_id);

  void restartContainer(
      const drogon::HttpRequestPtr& req,
      std::function<void(const drogon::HttpResponsePtr&)>&& callback,
      const std::string& container_id);

  void getContainerLogs(
      const drogon::HttpRequestPtr& req,
      std::function<void(const drogon::HttpResponsePtr&)>&& callback,
      const std::string& container_id);

 private:
  DockerClient dockerClient_;
};