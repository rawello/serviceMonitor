#pragma once
#include <json/value.h>
#include <string>
#include <vector>

class DockerClient {
 public:
  DockerClient(const std::string& socket_path = "/var/run/docker.sock");
  ~DockerClient();

  std::vector<Json::Value> GetContainers();
  bool StartContainer(const std::string& containerId);
  bool StopContainer(const std::string& containerId);
  bool RestartContainer(const std::string& containerId);
  std::string GetContainerLogs(const std::string& containerId);

 private:
  std::string socket_path_;
  std::string sendRequest(const std::string& method,
                          const std::string& path,
                          const std::string& body = "");
  Json::Value parseDockerResponse(const std::string& response);
};