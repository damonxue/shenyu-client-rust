@echo off
setlocal

cd %USERPROFILE%
git config --system core.longpaths true
git clone https://github.com/apache/shenyu
cd shenyu
mvn clean -Prelease -DskipTests package -pl ./shenyu-dist/shenyu-admin-dist -am -U

set targetName=shenyu-admin.tar.gz

rem Find the tar.gz file
for %%f in (shenyu-dist\shenyu-admin-dist\target\*.tar.gz) do (
    ren %%f "%targetName%"
)

tar -xzf shenyu-admin.tar.gz
cd shenyu-admin\bin
.\start.bat

endlocal