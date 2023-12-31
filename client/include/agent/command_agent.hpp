/**
 * @file command_agent.hpp Specialized agent
 *
 */

#ifndef REMOTEAGENT_INCLUDE_COMMAND_AGENT_HPP_
#define REMOTEAGENT_INCLUDE_COMMAND_AGENT_HPP_

#include <iostream>
#include <string>

#include "agent/agent_abstract.hpp"

namespace fleet_management::agent
{

    class CommandAgent : public AgentAbstract
    {

    public:
        CommandAgent(std::string &server_address, int id, bool debug = false) : AgentAbstract(CommandAgent::onMessage, server_address, id, debug){};
        ~CommandAgent() = default;
        CommandAgent(const CommandAgent &) = delete;
        CommandAgent(CommandAgent &&) = default;

        static void onMessage(AgentAbstract *agent, client *c, websocketpp::connection_hdl hdl, message_ptr msg);
    };

} // namespace fleet_management::agent
#endif // REMOTEAGENT_INCLUDE_COMMAND_AGENT_HPP_