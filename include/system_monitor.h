#pragma once
#include <cstdint>

class SystemMonitor
{
public:
    uint64_t GetUptime();
    double GetCpuLoad();
    uint64_t GetUsedMemory();
    uint64_t GetTotalMemory();
    double GetDiskUsage();
};