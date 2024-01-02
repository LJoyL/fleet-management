/**
 * @file command_agent.cpp Specialized agent
 *
 */

#include "agent/command_agent.hpp"

namespace fleet_management::agent
{

    void CommandAgent::onMessage(client *c, websocketpp::connection_hdl hdl, message_ptr msg)
    {
        std::cout << "on_message called with hdl: " << hdl.lock().get()
                  << " and message: " << msg->get_payload()
                  << std::endl;

        // websocketpp::lib::error_code ec;

        // c->send(hdl, msg->get_payload(), msg->get_opcode(), ec);
        // if (ec)
        // {
        //     std::cout << "Echo failed because: " << ec.message() << std::endl;
        // }
    };

} // namespace fleet_management::agent