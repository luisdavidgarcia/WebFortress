# Progress So Far

## Apache Files for Download

1. First Link: https://httpd.apache.org/security/vulnerabilities_13.html
2. Critial Search: https://www.google.com/search?client=firefox-b-1-d&q=Apache%2F1.3.31+vulnerabiliteis
3. Where to find old versions: https://archive.apache.org/dist/httpd/binaries/linux/

## Loosened Permissions

1. Had to loosen the following:
    - capabailities
    - mounts
    - resources
    - syscalls

## Key Testing Note

1. Similar to JuiceShop have to make the mountdir the uid/gid of the container, otherwise we can't write.

For instance notice the 10000 here:

```sh
drwxrwxr-x  8 sikeboy sikeboy 4096 Jun  5 22:48 dvwa_dir
drwxrwxr-x 32   10000   10000 4096 Jun  7 21:08 juice-shop
```

## How to Install dependencies

```sh
sudo apt install nmap
# Add the metasploitable install below
```

## How to Install Apache

Prior to this you have to make these in the host:

```sh
 sudo chown -R 10000:10000 /usr/local/apache2/
```

```sh
./configure --prefix=/usr/local/apache2 --with-pcre=/usr/local/pcre/make 
make
make install
```

## After must change the Listen and Server

```sh
nano /usr/local/apache2/conf/httpd.conf
```

Then in the file do:

```sh
Listen 172.18.0.2:3000
ServerName 172.18.0.2:3000
```

After saving and exiting the file execute these:

```sh
/usr/local/apache2/bin/apachectl -k restart
/usr/local/apache2/bin/apachectl -k start
```


## Simple fix to add gcc libraries

```sh
export PATH=$PATH:/usr/bin
```

## How to Install pcre (Must Be Done on LocalHost)

```sh
sudo apt-get install -y libpcre3 libpcre3-dev
```

## How to Install apr and apr-util (Apache Runtime)

Download both:

- apr-util-1.6.3.tar.gz
- apr-1.7.4.tar.gz

