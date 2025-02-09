#include <drogon/drogon.h>

int main()
{
    LOG_INFO << "Server running on port 8089";

    drogon::app()
        .loadConfigFile("config.json")
        .run();

    return 0;
}