@echo off

cd %USERPROFILE%
git config --system core.longpaths true
git clone https://github.com/apache/shenyu
cd shenyu
mvn clean -Prelease -DskipTests package -pl ./shenyu-dist/shenyu-admin-dist -am -U

set "targetName=shenyu-admin.tar.gz"
set sourcePath=%USERPROFILE%\shenyu\shenyu-dist\shenyu-admin-dist\target

for %%f in (%sourcePath%\apache-shenyu*.tar.gz) do (
    ren "%%f" "%targetName%"
    echo Renamed %%f to %targetName%
)

cd %sourcePath%
tar -xzf shenyu-admin.tar.gz
cd shenyu-admin\bin
.\start.bat
