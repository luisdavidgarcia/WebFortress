# Installation for WebGoat

## Step 0: Prereqs

First install the prequesites:

```sh
sudo apt-get update
sudo apt-get install openjdk-17-jdk -y
```

Follow this `Install_WebGoat.sh` script:

```sh
#!/bin/bash

# Create a directory for WebGoat
mkdir -p webgoat_dir

# Download WebGoat
wget https://github.com/WebGoat/WebGoat/releases/download/v2023.8/webgoat-2023.8.jar -O webgoat_dir/webgoat.jar

# Set permissions
chmod +x webgoat_dir/webgoat.jar

echo "WebGoat installed successfully in webgoat_dir"
```

## Step 1: Running WebGoat

To run WebGoat, use the following command within your container environment:

```sh
java -jar webgoat_dir/webgoat.jar --server.address=172.18.0.2 --server.port=8080
```

This command specifies the IP address and port for WebGoat to bind to.
Learning More About WebGoat

To learn more about WebGoat, explore the following resources:

- **[WebGoat GitHub Repository](https://github.com/WebGoat/WebGoat)**
- **[OWASP WebGoat Project Page](https://owasp.org/www-project-webgoat/)**
- **[Wiki WebGoat Documentation](https://github.com/WebGoat/WebGoat/wiki)**

## Potential Exploits and Learning

WebGoat includes various lessons that cover a wide range of security topics and vulnerabilities, including:

- SQL Injection
- Cross-Site Scripting (XSS)
- Insecure Deserialization
- Command Injection
- Directory Traversal

To practice and learn more about these exploits:

- Start WebGoat and navigate to http://172.18.0.2:8080/WebGoat in your browser.
- Login with the default credentials (usually guest/guest).
- Select a Lesson from the menu to start learning about specific vulnerabilities.
- Follow the Instructions in each lesson to exploit the vulnerabilities and understand the underlying security issues.

WebGoat provides a safe and legal environment to practice exploiting vulnerabilities, making it an excellent tool for learning web application security.