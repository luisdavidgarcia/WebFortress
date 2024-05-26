# DVWA Setup

## Prerequisites
- Ubuntu-based system
- Root privileges
- [DVWA Website](https://github.com/digininja/DVWA/tree/master)

## Installation Steps

### Install Required Packages
```sh
sudo apt-get update
sudo apt-get install -y apache2 mariadb-server mariadb-client php php-mysqli php-gd libapache2-mod-php
```

### Download and Set Up DVWA
1. Clone the DVWA repository:
    ```sh
    cd /var/www/html
    sudo git clone https://github.com/digininja/DVWA.git
    cd DVWA
    sudo cp config/config.inc.php.dist config/config.inc.php
    ```

2. Set up the database:
    ```sh
    sudo service mysql start
    sudo mysql -u root -e "CREATE DATABASE dvwa;"
    sudo mysql -u root -e "CREATE USER 'dvwa'@'localhost' IDENTIFIED BY 'p@ssw0rd';"
    sudo mysql -u root -e "GRANT ALL ON dvwa.* TO 'dvwa'@'localhost';"
    sudo mysql -u root -e "FLUSH PRIVILEGES;"
    ```

3. Configure DVWA:
    ```php
    sudo nano config/config.inc.php
    ```
    Ensure the database configuration is correct:
    ```php
    $_DVWA['db_server'] = '127.0.0.1';
    $_DVWA['db_port'] = '3306';
    $_DVWA['db_user'] = 'dvwa';
    $_DVWA['db_password'] = 'p@ssw0rd';
    $_DVWA['db_database'] = 'dvwa';
    $_DVWA['disable_authentication'] = true;
    $_DVWA['default_security_level'] = 'low';
    ```

4. Configure Apache for DVWA:
    ```sh
    sudo nano /etc/apache2/sites-available/000-default.conf
    ```
    Modify the configuration to serve DVWA:
    ```apache
    DocumentRoot /var/www/html/DVWA
    <Directory /var/www/html/DVWA>
        Options Indexes FollowSymLinks
        AllowOverride All
        Require all granted
    </Directory>
    ```

5. Enable Apache Modules and Restart:
    ```sh
    sudo a2enmod rewrite
    sudo systemctl restart apache2
    ```

### Step 3: Access DVWA
- Open your web browser and navigate to `http://localhost` or `http://<container_ip>`.
- Complete the DVWA setup by clicking on the “Create / Reset Database” button.

## General Tips and Advice

### Security Best Practices
- Regularly update your system and software to patch vulnerabilities.
- Use strong passwords for database users and administrative accounts.
- Ensure your container has limited access to host resources.

### Troubleshooting
- If you encounter issues with the database setup, check the `config.inc.php` file for correct database credentials.
- For Apache configuration issues, review the Apache error logs located at `/var/log/apache2/error.log`.

### Useful Commands
- Restart Apache: `sudo systemctl restart apache2`
- Check Apache status: `sudo systemctl status apache2`
- Access MySQL shell: `sudo mysql -u root`

## Container Setup

### Example Configuration File (`config.toml`)
```toml
debug = true
uid = 0
mount_dir = "./mountdir"
command = "/usr/sbin/apache2ctl -D FOREGROUND"
additional_paths = [
    "/lib64:/lib64",
    "/lib:/lib",
    "/usr/lib:/usr/lib",
    "/usr/lib64:/usr/lib64",
    "/bin:/bin",
    "/usr/bin:/usr/bin",
    "/usr/local/bin:/usr/local/bin",
    "/usr/lib/php:/usr/lib/php",
    "/etc/apache2:/etc/apache2",
    "/var/www/html:/var/www/html",
    "/var/lib/mysql:/var/lib/mysql",
    "/usr/sbin:/usr/sbin"
]
```

### Start the Container
```sh
sudo ./my_container_binary --config config.toml
```

### Testing DVWA in Your Container
- **Using Nmap for Network Scanning:**
    ```sh
    nmap -p 80 localhost
    ```
- **Using Metasploit to Exploit DVWA:**
    ```sh
    msfconsole
    use exploit/unix/webapp/phpmyadmin_2_8_0_3
    set RHOST localhost
    set RPORT 80
    run
    ```
- **Using Burp Suite for Web Application Testing:**
    - Configure Burp Suite to proxy traffic to your DVWA instance and perform various web vulnerability tests.

- **Using OWASP ZAP for Automated Scanning:**
    - Configure OWASP ZAP to scan the DVWA instance running inside your container.