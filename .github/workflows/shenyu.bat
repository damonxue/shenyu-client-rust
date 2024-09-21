set url = "https://github.com/libarchive/libarchive/releases/download/v3.5.2/libarchive-3.5.2-win64.zip"
set output = "tar.zip"
Invoke-WebRequest -Uri %url% -OutFile %output%
Expand-Archive -Path %output% -DestinationPath %GITHUB_WORKSPACE%\tar
Add-Content -Path %GITHUB_PATH% -Value "%GITHUB_PATH%\tar\bin"

git config --system core.longpaths true
git clone https://github.com/apache/shenyu
cd shenyu/shenyu-dist/shenyu-admin-dist
set pomFile="pom.xml"
powershell -Command "(Get-Content %pomFile%) -replace '<finalName>.*</finalName>', '<finalName>shenyu-admin</finalName>' | Set-Content %pomFile%"
cd ../../
mvn clean -Prelease -Dmaven.javadoc.skip=true -B -Drat.skip=true -Djacoco.skip=true -DskipITs -DskipTests package -pl ./shenyu-dist/shenyu-admin-dist -am -U
cd shenyu-dist\shenyu-admin-dist\target
powershell -Command "Expand-Archive -Path shenyu-admin-admin-bin.tar.gz -DestinationPath ."
cd shenyu-admin-admin-bin\bin
.\start.bat
