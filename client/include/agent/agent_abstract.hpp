/**
 * @file agent_abstract.hpp Abstract class for a standard agent
 *
 */

#ifndef REMOTEAGENT_INCLUDE_AGENT_ABSTRACT_HPP_
#define REMOTEAGENT_INCLUDE_AGENT_ABSTRACT_HPP_

#include <iostream>
#include <string>
#include <functional>

#include <nlohmann/json.hpp>
#include <HTTPRequest.hpp>
#include <websocketpp/config/asio_no_tls_client.hpp>
#include <websocketpp/client.hpp>

namespace fleet_management::agent
{

    using json = nlohmann::json;

    typedef websocketpp::client<websocketpp::config::asio_client> client;
    typedef websocketpp::config::asio_client::message_type::ptr message_ptr;

    using websocketpp::lib::bind;

    class AgentAbstract
    {

    public:
        std::string m_server_address;

        int m_agent_id;
        std::string m_agent_type;
        std::string m_agent_name;
        std::string m_agent_description;
        std::string m_agent_status;

        client m_c;
        bool m_registered = false;

        AgentAbstract(std::function<void(AgentAbstract *agent, client *c, websocketpp::connection_hdl hdl, message_ptr msg)> onMessageFct,
                      std::string const &server_address, int id,
                      std::string const &type = std::string("default_type"),
                      std::string const &name = std::string("dummy"),
                      std::string const &desc = std::string());

        AgentAbstract(AgentAbstract const &) = delete;
        AgentAbstract(AgentAbstract &&) = default;

        virtual ~AgentAbstract() = default;

        /**
         * @brief Callback function called when a message is received
         */
        // virtual void onMessage(client *c, websocketpp::connection_hdl hdl, message_ptr msg) = 0;

        /**
         * @brief Start the ws connection
         */
        void run();

    private:
        /**
         * @brief Register the agent to the given server url
         */
        bool registerToServer(std::string &server_address, std::string &ws_url);
    };

} // namespace fleet_management::agent
#endif // REMOTEAGENT_INCLUDE_AGENT_ABSTRACT_HPP_