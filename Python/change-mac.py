#!/usr/bin/env python

import subprocess

interface = input("Enter the interface you would like to change > ")
new_mac = input("Enter the new MAC of your choice > ")

print("[+] Changing MAC address for " + interface + " to " + new_mac)

subprocess.call("ifconfig " + interface + " down", shell=True)
subprocess.call("ifconfig " + interface + "hw ether " + new_mac, shell=True)
subprocess.call("ifconfig " + interface + " up", shell=True)
