# Web Media Controller Windows (wmc-win)

wmc-win is a native Windows extension for controlling media playback within web browsers.
It uses the [Web Media Controller](https://github.com/f1u77y/web-media-controller) browser extension to interface with Firefox ([Mozilla Add-on](https://addons.mozilla.org/en-US/firefox/addon/web-media-controller/)) and Chrome/Chromium.

## Installation
A Windows MSI package is available on the [GitHub releases page](https://github.com/brookst/wmc-win/releases/latest).
Run the MSI to install wmc-win and configure web browsers to find it.
Don't forget to install [Web Media Controller](https://github.com/f1u77y/web-media-controller) into your browser!

## Usage
wmc-win is a native extension started by [Web Media Controller](https://github.com/f1u77y/web-media-controller) in the web browser.
Currently, wmc-win listens for the following keys:

* [`VK_MEDIA_NEXT_TRACK`](https://referencesource.microsoft.com/windowsbase/R/5069862b166d95f1.html)
* [`VK_MEDIA_PREV_TRACK`](https://referencesource.microsoft.com/windowsbase/R/5df9df218d733fe0.html)
* [`VK_MEDIA_STOP`](https://referencesource.microsoft.com/windowsbase/R/ff4207f203a7a13c.html)
* [`VK_MEDIA_PLAY_PAUSE`](https://referencesource.microsoft.com/windowsbase/R/2493a61114ea6df8.html)

## Notes

### Continuous Integration
[![AppVeyor](https://ci.appveyor.com/api/projects/status/github/brookst/wmc-win?branch=master&svg=true)](https://ci.appveyor.com/project/brookst/wmc-win)  
PRs, `master` and tags are built on [Appveyor](https://ci.appveyor.com/project/brookst/wmc-win).
Tags get packaged with [`cargo-wix`](https://github.com/volks73/cargo-wix) and deployed to the [GitHub release page](https://github.com/brookst/wmc-win/releases/latest).

### Native Messaging
Firefox and Chrome/Chromium locate native messaging binaries from the Windows registry, via JSON manifests.
Set the appropriate registry keys with the following:

    reg add HKEY_CURRENT_USER\SOFTWARE\Mozilla\NativeMessagingHosts\me.f1u77y.web_media_controller /t REG_SZ /d C:\Users\user\AppData\Local\wmc-win\me.f1u77y.web_media_controller.firefox.json
    reg add HKEY_CURRENT_USER\SOFTWARE\Google\Chrome\NativeMessagingHosts\me.f1u77y.web_media_controller /t REG_SZ /d C:\Users\user\AppData\Local\wmc-win\me.f1u77y.web_media_controller.chromium.json

Make sure the `path` in the JSON manifest does point to the binary location.
