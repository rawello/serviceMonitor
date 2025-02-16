#include "../include/system_monitor.h"
#include <bits/this_thread_sleep.h>
#include <sys/statvfs.h>
#include <sys/sysinfo.h>
#include <unistd.h>
#include <fstream>
#include <iostream>
#include <sstream>
#include <vector>

uint64_t SystemMonitor::GetUptime() {
  struct sysinfo info;
  sysinfo(&info);
  return info.uptime;
}

double SystemMonitor::GetCpuLoad() {
  std::ifstream proc_stat("/proc/stat");
  if (!proc_stat.is_open()) {
    std::cerr << "Failed to open /proc/stat" << std::endl;
    return -1.0;
  }

  std::vector<std::pair<uint64_t, uint64_t>> first_read;
  std::string line;

  while (std::getline(proc_stat, line)) {
    if (line.find("cpu") == 0 && line.find("cpu ") != 0) {
      std::istringstream iss(line);
      std::string cpu_name;
      uint64_t user, nice, system, idle, iowait, irq, softirq, steal;
      iss >> cpu_name >> user >> nice >> system >> idle >> iowait >> irq >>
          softirq >> steal;

      uint64_t total =
          user + nice + system + idle + iowait + irq + softirq + steal;
      uint64_t idle_time = idle + iowait;

      first_read.emplace_back(total, idle_time);
    }
  }

  std::this_thread::sleep_for(std::chrono::milliseconds(1000));

  std::vector<std::pair<uint64_t, uint64_t>> second_read;
  proc_stat.clear();
  proc_stat.seekg(0);

  while (std::getline(proc_stat, line)) {
    if (line.find("cpu") == 0 && line.find("cpu ") != 0) {
      std::istringstream iss(line);
      std::string cpu_name;
      uint64_t user, nice, system, idle, iowait, irq, softirq, steal;
      iss >> cpu_name >> user >> nice >> system >> idle >> iowait >> irq >>
          softirq >> steal;

      uint64_t total =
          user + nice + system + idle + iowait + irq + softirq + steal;
      uint64_t idle_time = idle + iowait;

      second_read.emplace_back(total, idle_time);
    }
  }

  double total_load = 0.0;
  for (size_t i = 0; i < first_read.size(); ++i) {
    uint64_t total_diff = second_read[i].first - first_read[i].first;
    uint64_t idle_diff = second_read[i].second - first_read[i].second;

    if (total_diff > 0) {
      total_load += (double)(total_diff - idle_diff) / total_diff;
    }
  }

  return total_load * 100.0;
}

uint64_t SystemMonitor::GetUsedMemory() {
  struct sysinfo info;
  sysinfo(&info);
  return info.totalram - info.freeram;
}

uint64_t SystemMonitor::GetTotalMemory() {
  struct sysinfo info;
  sysinfo(&info);
  return info.totalram;
}

double SystemMonitor::GetDiskUsage() {
  struct statvfs stat;
  statvfs("/", &stat);
  return (1.0 - (double)stat.f_bavail / stat.f_blocks) * 100.0;
}