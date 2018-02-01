# Servo Display Manager
A themeable login/display manager for wayland sessions using Mozilla's Servo browser.

### Goals
* A high performance, light-weight display manager
* The ability to swap in and out different HTML & Javascript login themes
* An easy to use theme api provided in javascript

### Similar Softwares
* [RDM](https://github.com/1wilkens/rdm) - A rust based display manager
* [SDDM](https://github.com/sddm/sddm) - A display manager with wayland support

### Developing
1. Install [dependencies](#dependencies).
1. Clone github repository and enter the new directory.
    ```bash
    git clone https://github.com/FallingSnow/sdm.git && cd sdm
    ```
1. Compile sdm.
    ```bash
    cargo build --debug
    ```
1. Run sdm.
    ```bash
    target/debug/sdm
    ```

### Dependencies
* [Rust](https://www.rust-lang.org/en-US/)
* [Cargo](https://doc.rust-lang.org/cargo/)

### Roadmap
* [ ] Display a window
* [ ] Embed Servo
* [ ] Theme API - Use [web-greeters api](https://doclets.io/Antergos/web-greeter/stable) as a template
    * [ ] Supported APIs
        * [ ] poweroff
        * [ ] reboot
        * [ ] hibernate
        * [ ] sleep
    * [ ] Login
        * [ ] Authenticate(user, password)
        * [ ] Start session(session)
    * [ ] User List
        * [ ] Logged in
        * [ ] Avatar
    * [ ] Utilities
        * [ ] Dirlist(directory) - Get a list of a directory
        * [ ] Config(key) - Get the value of key under the [theme] section in config
    * [ ] Power
        * [ ] Poweroff()
        * [ ] Reboot()
        * [ ] Hibernate()
        * [ ] Sleep()
    * [ ] Timezone

### Notes
* Getting a list of wayland sessions -
A list of supported wayland sessions can be founds in `/usr/share/wayland-sessions`.
* Starting a wayland session -active
Sessions can be started via `XDG_SESSION_TYPE=wayland gnome-session`.
* [users](https://github.com/ogham/rust-users) - Can be used to get users (among other things) to send to theme api
* [parsswd](https://github.com/kstep/parsswd) - Native passwd parser to go through users
* [rust-systemd](https://github.com/jmesmon/rust-systemd) - [Added login support](https://github.com/jmesmon/rust-systemd/pull/45) for smithay (I wonder if we can use smithay to create a compositor that can be shared with Fireplace)

### Learning Resources
* [Logind](https://www.freedesktop.org/wiki/Software/systemd/logind/) - systemd service that keeps track of user logins and seats
* [pam_systemd](https://www.freedesktop.org/software/systemd/man/pam_systemd.html) - pam library to register users for login sessions
* [Smithay](https://github.com/Smithay/smithay) - Library for creating wayland compositors
* [Fireplace](https://github.com/Drakulix/fireplace) - A window manager for wayland (can be used to test this display manager's ability to open wayland sessions)
* [Display Manager](https://github.com/gsingh93/display-manager) - A simple display manager written in C
