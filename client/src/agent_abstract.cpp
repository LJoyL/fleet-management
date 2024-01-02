#include "agent/agent_abstract.hpp"

/**
 * @file agent_abstract.cpp Abstract class for a standard agent
 *
 */

namespace fleet_management::agent
{

    AgentAbstract::AgentAbstract(std::function<void(AgentAbstract *agent, client *c, websocketpp::connection_hdl hdl, message_ptr msg)> onMessageFct, std::string const &server_address, int id, std::string const &type, std::string const &name, std::string const &desc)
        : m_server_address(server_address),
          m_agent_id(id),
          m_agent_type(type),
          m_agent_name(name),
          m_agent_description(desc)
    {
        std::string ws_url;
        if (registerToServer(m_server_address, ws_url))
        {
            try
            {
                // Set logging to be pretty verbose (everything except message payloads)
                m_c.set_access_channels(websocketpp::log::alevel::all);
                m_c.clear_access_channels(websocketpp::log::alevel::frame_payload);

                // Initialize ASIO
                m_c.init_asio();

                // Register our message handler
                m_c.set_message_handler(bind(onMessageFct, this, &m_c, websocketpp::lib::placeholders::_1, websocketpp::lib::placeholders::_2));

                websocketpp::lib::error_code ec;
                client::connection_ptr con = m_c.get_connection(ws_url, ec);
                if (ec)
                {
                    std::cout << "could not create connection because: " << ec.message() << std::endl;
                    return;
                }

                // Note that connect here only requests a connection. No network messages are
                // exchanged until the event loop starts running in the next line.
                m_c.connect(con);

                m_registered = true;
            }
            catch (websocketpp::exception const &e)
            {
                std::cout << e.what() << std::endl;
            }
        }
        else
        {
            m_registered = false;
        }
    };

    void AgentAbstract::run()
    {
        // Start the ASIO io_service run loop
        // this will cause a single connection to be made to the server. c.run()
        // will exit when this connection is closed.
        if (m_registered)
        {
            m_c.run();
        }
        else
        {
            std::cout << "Agent not registered, cannot run" << '\n';
        }
    };

    bool AgentAbstract::registerToServer(std::string &server_address, std::string &ws_url)
    {
        try
        {
            http::Request request{server_address + "/register"};

            // Create request body
            json req_body;
            req_body["user_id"] = m_agent_id;
            req_body["topic"] = "default";

            std::cout
                << "Sending request POST: " << req_body.dump() << '\n';
            const auto response = request.send("POST", req_body.dump(), {{"Content-Type", "application/json"}});

            if (response.status.code != 200)
            {
                std::cerr << "Error: " << response.status.code << '\n';
                return false;
            }

            json res_body = json::parse(response.body.begin(), response.body.end());
            std::cout << "Received respond: " << res_body << '\n'; // print the result

            // Assign the ws_address to the agent
            ws_url = res_body["url"];
            return true;
        }
        catch (const std::exception &e)
        {
            std::cerr << "Error: Cannot register, " << e.what() << '\n';
            return false;
        }
    };

} // namespace fleet_management::agent