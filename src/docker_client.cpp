#include "docker_client.h"
#include <curl/curl.h>
#include <json/reader.h>
#include <trantor/utils/Logger.h>
#include <iostream>
#include <sstream>
#include <bits/this_thread_sleep.h>

size_t WriteCallback(void* contents, size_t size, size_t nmemb, void* userp) {
  ((std::string*)userp)->append((char*)contents, size * nmemb);
  return size * nmemb;
}

DockerClient::DockerClient(const std::string& socket_path)
    : socket_path_(socket_path) {
  curl_global_init(CURL_GLOBAL_ALL);
}

DockerClient::~DockerClient() {
  curl_global_cleanup();
}

std::string DockerClient::sendRequest(const std::string& method,
                                      const std::string& path,
                                      const std::string& body) {
  CURL* curl;
  CURLcode res;
  std::string response;

  curl = curl_easy_init();
  if (!curl) {
    throw std::runtime_error("Failed to initialize libcurl");
  }

  try {
    std::string url = "http://localhost" + path;

    curl_easy_setopt(curl, CURLOPT_URL, url.c_str());
    curl_easy_setopt(curl, CURLOPT_CUSTOMREQUEST, method.c_str());
    curl_easy_setopt(curl, CURLOPT_UNIX_SOCKET_PATH, socket_path_.c_str());

    if (!body.empty()) {
      curl_easy_setopt(curl, CURLOPT_POSTFIELDS, body.c_str());
    }

    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, WriteCallback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &response);

    res = curl_easy_perform(curl);
    if (res != CURLE_OK) {
      throw std::runtime_error("Curl error: " +
                               std::string(curl_easy_strerror(res)));
    }

    long http_code = 0;
    curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);
    if (http_code != 200 && http_code != 204) {
      throw std::runtime_error("HTTP error: " + std::to_string(http_code) +
                               ", Response: " + response);
    }
  } catch (...) {
    curl_easy_cleanup(curl);
    throw;
  }

  curl_easy_cleanup(curl);
  return response;
}

Json::Value DockerClient::parseDockerResponse(const std::string& response) {
  Json::Value root;
  Json::CharReaderBuilder builder;
  std::string errors;
  std::istringstream jsonStream(response);

  if (!Json::parseFromStream(builder, jsonStream, &root, &errors)) {
    throw std::runtime_error("Failed to parse Docker response: " + errors);
  }
  return root;
}

std::vector<Json::Value> DockerClient::GetContainers() {
  std::string response = sendRequest("GET", "/containers/json?all=true");
  Json::Value json = parseDockerResponse(response);

  std::vector<Json::Value> containers;
  for (const auto& item : json) {
    containers.push_back(item);
  }
  return containers;
}

bool DockerClient::StartContainer(const std::string& containerId) {
  std::string path = "/containers/" + containerId + "/start";
  std::string response = sendRequest("POST", path);
  return true;
}

bool DockerClient::StopContainer(const std::string& containerId) {
  std::string path = "/containers/" + containerId + "/stop?t=10";
  std::string response = sendRequest("POST", path);
  return true;
}

bool DockerClient::RestartContainer(const std::string& containerId) {
  StopContainer(containerId);
  std::this_thread::sleep_for(std::chrono::seconds(2));
  return StartContainer(containerId);
}

std::string DockerClient::GetContainerLogs(const std::string& containerId) {
  std::string path =
      "/containers/" + containerId + "/logs?stderr=1&stdout=1&tail=100";
  return sendRequest("GET", path);
}