/**
 * @file command_agent.cpp Specialized agent
 *
 */

#include "agent/command_agent.hpp"

namespace fleet_management::agent
{

    void CommandAgent::onMessage(AgentAbstract *agent, client *c, websocketpp::connection_hdl hdl, message_ptr msg)
    {
        std::cout << "on_message called with hdl: " << hdl.lock().get()
                  << " and message: " << msg->get_payload()
                  << std::endl;

        // parse message into json
        json j = json::parse(msg->get_payload());

        if (j.contains("delay"))
        {
            int delay = j["delay"];
            int rand_delay = rand() % delay + 1;
            std::cout << "waiting delay: " << rand_delay << std::endl;
            sleep(rand_delay);
        }

        // send respond to client, simply echo
        websocketpp::lib::error_code ec;

        c->send(hdl, msg->get_payload() + " from " + std::to_string(agent->m_agent_id), msg->get_opcode(), ec);
        if (ec)
        {
            std::cout << "Echo failed because: " << ec.message() << std::endl;
        }
    };

} // namespace fleet_management::agent