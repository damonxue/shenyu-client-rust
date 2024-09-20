#!/usr/bin/env sh

cd ~
wget https://archive.apache.org/dist/shenyu/2.6.0/apache-shenyu-2.6.0-admin-bin.tar.gz
tar -zxvf apache-shenyu-2.6.0-admin-bin.tar.gz
cd ~/apache-shenyu-2.6.0-admin-bin/bin
if [ "${OS}" = "windows-latest" ]; then
  sh start.bat
else
  sh start.sh
fi
