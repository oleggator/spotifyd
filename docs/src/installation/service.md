# Running as a service

Most people want to have `spotifyd` always running in the background. The preferred method to do this depends on the OS you're using.

## Linux

If you installed `spotifyd` directly from your distribution, chances are that the installation includes a service definition such that you can run

```console
$ systemctl --user start spotifyd # start spotifyd using systemd
$ systemctl --user enable --now spotifyd # start spotifyd and enable starting on login
```

If you installed `spotifyd` some other way, head over to [the advanced section](../advanced/systemd.md) for further instructions.

## macOS

If you installed `spotifyd` using brew, the following commands should do the trick

```console
$ brew services run spotifyd # start the service once
$ brew services start spotifyd # start spotifyd and enable starting on boot
```

If you installed `spotifyd` without brew, [the advanced section](../advanced/launchd.md) has got you covered.

## FreeBSD

When installed via the package manager, the following commands are available:

```console
$ sudo service spotifyd onestart # start spotifyd once
$ sudo sysrc spotifyd_enable=YES # enable starting spotifyd on boot
```

## Windows

`spotifyd` can run as a Windows service. The scripts in `contrib/windows` install and remove the service. Open a terminal as administrator and run:

```console
> powershell -ExecutionPolicy Bypass -File contrib\windows\install-service.ps1
> powershell -ExecutionPolicy Bypass -File contrib\windows\uninstall-service.ps1
```

By default, the install script uses `target\release\spotifyd.exe` and keeps the configuration and cache in `C:\ProgramData\spotifyd`. To use other paths, give the `-BinaryPath`, `-ConfigPath` and `-CachePath` parameters.

To register the service manually, use `sc.exe`:

```console
> sc.exe create spotifyd binPath= "C:\spotifyd\spotifyd.exe --windows-service --config-path C:\spotifyd\spotifyd.conf --cache-path C:\spotifyd\cache" start= auto
> sc.exe start spotifyd
```

The service runs as the `LocalSystem` account by default, so it cannot find the configuration and cache in your user folders. Give both paths on the command line, as in the example above.
If you log in with OAuth, run `spotifyd authenticate --cache-path C:\spotifyd\cache` once before you start the service.

Some computers have virtual network adapters, for example for Hyper-V, WSL or a VPN. If the device is then not visible on your network, set `zeroconf_ip` in the configuration file to the IP address of your LAN adapter.

The service writes its log to `C:\Windows\System32\config\systemprofile\AppData\Local\spotifyd\.spotifyd.log`.

To stop and remove the service, run:

```console
> sc.exe stop spotifyd
> sc.exe delete spotifyd
```
