#include "../include/system_monitor.h"
#include <fstream>
#include <sstream>
#include <sys/sysinfo.h>
#include <unistd.h>
#include <sys/statvfs.h>

uint64_t SystemMonitor::GetUptime()
{
    struct sysinfo info;
    sysinfo(&info);
    return info.uptime;
}

double SystemMonitor::GetCpuLoad()
{
    std::ifstream proc_stat("/proc/stat");
    std::string line;
    std::getline(proc_stat, line);
    std::istringstream iss(line);
    std::string cpu;
    uint64_t user, nice, system, idle;
    iss >> cpu >> user >> nice >> system >> idle;

    uint64_t total = user + nice + system + idle;
    sleep(1);

    proc_stat.seekg(0);
    std::getline(proc_stat, line);
    iss.str(line.substr(5));
    iss >> user >> nice >> system >> idle;
    uint64_t total2 = user + nice + system + idle;

    return (total2 - total) / 100.0;
}

uint64_t SystemMonitor::GetUsedMemory()
{
    struct sysinfo info;
    sysinfo(&info);
    return info.totalram - info.freeram;
}

uint64_t SystemMonitor::GetTotalMemory()
{
    struct sysinfo info;
    sysinfo(&info);
    return info.totalram;
}

double SystemMonitor::GetDiskUsage()
{
    struct statvfs stat;
    statvfs("/", &stat);
    return (1.0 - (double)stat.f_bavail / stat.f_blocks) * 100.0;
}