cmake_minimum_required(VERSION 3.13)  # CMake version check
project(fleet_management)
set(CMAKE_CXX_STANDARD 17)

set(SOURCE_FILES 
    agent_abstract.cpp
    command_agent.cpp
)

set(APPS
    commandAgentApp.cxx
    multiCommandAgentApp.cxx
)

set(TEST_APPS
    clientTestApp.cxx
)

list(TRANSFORM SOURCE_FILES PREPEND src/)
list(TRANSFORM APPS PREPEND apps/)
list(TRANSFORM TEST_APPS PREPEND test/apps/)

include_directories(include)
include_directories(include/websocketpp)
include_directories(include/json/include)
include_directories(include/HTTPRequest/include)

# Add executable for every app
foreach(APP ${APPS})
    get_filename_component(APP_NAME ${APP} NAME_WE)
    add_executable(${APP_NAME} ${SOURCE_FILES} ${APP})
endforeach()

foreach(APP ${TEST_APPS})
    get_filename_component(APP_NAME ${APP} NAME_WE)
    add_executable(${APP_NAME} ${APP})
endforeach()