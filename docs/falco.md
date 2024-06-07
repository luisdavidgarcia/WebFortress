# Falco

We installed Falco as means to measures our containers security.

We followed the installation steps outlined here and utilized `Kmod` with automatic configuration as they suggested: https://falco.org/docs/install-operate/installation/

## Resolve Error when Using Kmod

Needed to utilized the command: 

```sh
sudo apt-get install -y build-essential linux-headers-$(uname -r) dkms
```

## Next Steps

Run the following commands to get Falco running:

```sh
sudo systemctl enable falco-modern-bpf.service
sudo systemctl start falco-modern-bpf.service
```

## Note on VSCODE

When I adjust the view of the terminal I mess up the JuiceShop application?