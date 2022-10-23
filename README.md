# MSAL Login Forwarder

If you're running a Linux virtual machine on Windows, this tiny utility brings native Windows AAD interactive login experience to Linux.
You no longer have to worry about device restrictions in Linux. Any interactive login requests can be forwarded to a browser on Windows, thus
leverage your existing login state.

> Disclaimer: Use at your own risk.

## Windows side usage

Server side works a forwarder and browser invoker.

1. Put a config file at `%HOME%/msal-login-forwarder.toml`:

```toml
bind = "0.0.0.0:9080"
```

2. Run `server.exe`.

## Linux side usage

Client side works as a fake browser to receive requests. That means you need to cheat MSAL SDK to pretend our client is a browser.

First you need to put a config file at `$HOME/.config/msal-login-forwarder.toml`:

```toml
# Replace the address with your server address
server = "192.168.98.1:9080"
```

### Python clients / Azure CLI

MSAL in Python can only recognize `/usr/bin/microsoft-edge` as a compliant browser. You can create a symlink to `client`.

```bash
sudo ln -s /path/to/client /usr/bin/microsoft-edge
```

Then you can use `az login` to initiate an interactive login. A browser will automatically pop up at Windows side.

### Go clients

MSAL in Go tries `xdg-open`/`x-www-browser`/`www-browser` in your PATHs. I suggest you override the latter two.

## License

MIT.
