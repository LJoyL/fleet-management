/**
 * @file MultiCommandAgentApp.cxx Test app command agent
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
        // CommandAgent agent(url, id);

        // Create fleet of agents
        std::deque<CommandAgent> fleet;
        for (int i = 0; i < 10; i++)
        {
            fleet.emplace_back(url, i);
        }

        // Run every agent in a thread
        std::vector<std::thread> threads;
        for (auto &agent : fleet)
        {
            std::thread t([&agent]()
                          { agent.run(); });
            threads.push_back(std::move(t));
        }

        // Wait for all agents to finish
        for (auto &t : threads)
        {
            t.join();
        }
    }
    catch (const std::exception &e)
    {
        std::cerr << e.what() << '\n';
    }

    return 0;
}