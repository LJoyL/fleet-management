/**
 * @file commandAgentApp.cxx Test app command agent
 *
 */

#include "agent/command_agent.hpp"

#include <iostream>
#include <string>
#include <filesystem>
#include <cassert>
#include <stdexcept>

using namespace fleet_management::agent;

int main(int argc, char *argv[])
{
    int id = 1;
    std::string url = "http://localhost:8000";

    if (argc == 2)
    {
        id = std::stoi(argv[1]);
    }
    try
    {
        CommandAgent agent(url, id, true);
        agent.run();
    }
    catch (const std::exception &e)
    {
        std::cerr << e.what() << '\n';
    }

    return 0;
}