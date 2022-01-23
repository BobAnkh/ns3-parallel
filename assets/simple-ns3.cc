/* -*- Mode:C++; c-file-style:"gnu"; indent-tabs-mode:nil; -*- */
#include "ns3/core-module.h"

using namespace ns3;

NS_LOG_COMPONENT_DEFINE("SimpleSimulator");

int main(int argc, char *argv[])
{
    NS_LOG_UNCOND("A Simple NS3 Example of ns3-parallel");
    std::string app_name;

    uint32_t sim_time = 10;
    uint32_t policy = 0;

    /* Command line arguments */
    CommandLine cmd(__FILE__);
    cmd.AddValue("app-name", "The name of apps", app_name);
    cmd.AddValue("sim-time", "Total duration of the simulation (in s). Default to 10", sim_time);
    cmd.AddValue("policy", "Policy of program", policy);
    cmd.Parse(argc, argv);

    NS_LOG_UNCOND("[Param] app_name: " << app_name << " policy: " << policy << " sim_time: " << sim_time);
    Simulator::Stop(Seconds(sim_time + 1));
    Simulator::Run();
    Simulator::Destroy();
}
