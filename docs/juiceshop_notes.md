# General Notes

Had to basically follow the instructions from the [repo](https://github.com/juice-shop/juice-shop)

One main thing though is that after cloning you have to:

```bash
cd juice-shop
mkdir -p ./.npm/_logs
```

Also for `env_args` in `child.rs` you have to set them to this:

```bash
    let env_vars: Vec<CString> = vec![
        CString::new("LD_LIBRARY_PATH=/usr/lib:/usr/lib64:/usr/local/lib").unwrap(),
        CString::new("HOME=/").unwrap(),
    ];
```

Also had to give juice-shop permissions of the direcotry with:

```sh
sudo chown -R 10000:10000 juice-shop/
```