cmake_minimum_required(VERSION 3.15)
project(server_monitor)

include(${CMAKE_BINARY_DIR}/${CMAKE_BUILD_TYPE}/generators/conan_toolchain.cmake)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -frtti")

file(GLOB SOURCES src/*.cpp src/controllers/*.cpp)
file(GLOB HEADERS include/*.h include/controllers/*.h)

find_package(Drogon REQUIRED)
find_package(CURL REQUIRED)

add_executable(server_monitor ${SOURCES} ${HEADERS})

target_include_directories(server_monitor PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)

file(COPY ${CMAKE_SOURCE_DIR}/config.json DESTINATION ${CMAKE_BINARY_DIR}/${CMAKE_BUILD_TYPE}/generators/)

target_link_libraries(server_monitor PRIVATE
    Drogon::Drogon     
    CURL::libcurl
    pthread
)