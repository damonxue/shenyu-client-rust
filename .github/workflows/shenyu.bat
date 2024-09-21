cd %USERPROFILE%
git config --system core.longpaths true
git clone https://github.com/apache/shenyu
cd shenyu/shenyu-dist/shenyu-admin-dist
(Get-Content /shenyu-admin-dist.pom) -replace '<finalName>.*</finalName>', '<finalName>shenyu-admin</finalName>' | Set-Content /shenyu-admin-dist.pom
cd %USERPROFILE%\shenyu
mvn clean -Prelease -Dmaven.javadoc.skip=true -B -Drat.skip=true -Djacoco.skip=true -DskipITs -DskipTests package -pl ./shenyu-dist/shenyu-admin-dist -am -U
cd shenyu-dist\shenyu-admin-dist\target
tar -xzf shenyu-admin.tar.gz
cd shenyu-admin\bin
.\start.bat
