# Zbus low level examples

This project contains a few zbus low level examples to interact with DBus services like notificaiton, systemd, geoclue, etc.

### Testing the main binary:

1. Introspection

```sh
❯ busctl --user introspect org.zbus.Greeter /org/zbus/Greeter
NAME                                TYPE      SIGNATURE RESULT/VALUE  FLAGS
org.freedesktop.DBus.Introspectable interface -         -             -
.Introspect                         method    -         s             -
org.freedesktop.DBus.Peer           interface -         -             -
.GetMachineId                       method    -         s             -
.Ping                               method    -         -             -
org.freedesktop.DBus.Properties     interface -         -             -
.Get                                method    ss        v             -
.GetAll                             method    s         a{sv}         -
.Set                                method    ssv       -             -
.PropertiesChanged                  signal    sa{sv}as  -             -
org.zbus.Greeter1                   interface -         -             -
.GoAway                             method    -         -             -
.SayHello                           method    s         s             -
.GreeterName                        property  s         "ZbusGreeter" emits-change writable
.GreetedEveryone                    signal    -         -             -
```

2. Calling the service via `busctl`

```sh
❯ busctl --user call \
	org.zbus.Greeter \
	/org/zbus/Greeter \
	org.zbus.Greeter1 \
	SayHello s "zbus"
s "Hello zbus!"
```
