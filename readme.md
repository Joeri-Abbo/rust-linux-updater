#Automatic updater

What does this package do?
For each device in the devices.json it automatic updates the devices when the ./run.sh file is triggerd

Install
- First clone this repo ```git  clone git@github.com:Joeri-Abbo/linux-updater.git```
- cd in to the repo ```cd linux-updater```
- Run ``` chmod +x setup.sh``` so the script can install the needed software
- Run ```./setup.sh```
- Open the generated ```devices.json``` file and fil in your devices
- To start a update process run ```./run.sh```
- Run the command with a secudle like crontab.

Updating this package?
- Run ```./update.sh``` inside the repo