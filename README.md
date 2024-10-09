# Drexel CT 201 InfoSec Project

The final project of CT 201 is to select a hack where the target was specifically information/data.
I selected the [2013 Target Data Breach](https://www.commerce.senate.gov/services/files/24d3c229-4f2f-405d-b8db-a3a67f183883)
Hackers collected the financial and personal information for as many as 110 million Target customers.

It was reported part of the hack was facilitated by the [BlackPOS malware](https://en.wikipedia.org/wiki/BlackPOS) to steal customer information as they scanned their cards on Targets point of sale systems.
This malware enumerates process runnign on a windows host and hooks into any process named `pos.exe` then scans its memory for credit card information.

This project seeks to re-engineer the BlackPOS malware and demonstrate how it works.

## mock_blackpos

The mock_blackpos directory contains the Rust source code for a program that performs the base functionality of BlackPOS:
- Enumerates system processes
- Hooks into any pos.exe process
- Uses a handle to the process to repeatedly scan the processes memory for credit card information

This program simply prints the captured information to stdout, presumably the BlackPOS malware in the Target breach bundled the captured information and shipped it off of Targets network.

## victim_pos

The victim_pos directory contains the Go source code for a demonstrative Point-of-Sale system for the mock_blackpos malware to hook into.
This POS system:
- Searches the local system for a MSR605* card reader and if one is found attaches it to the POS system
- Displays a GUI to return only the name found on scanned credit cards

## Demonstration

[![A demonstration of the project can be found here:](https://img.youtube.com/vi/-gjtjLrjgek/maxresdefault.jpg)](https://youtu.be/-gjtjLrjgek)
