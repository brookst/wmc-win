# Web Media Controller Windows (wmc-win)

wmc-win is a native Windows application for controlling media playback within web browsers.
It uses the [Web Media Controller](https://github.com/f1u77y/web-media-controller) browser extension to interface with Firefox ([Mozilla Add-on](https://addons.mozilla.org/en-US/firefox/addon/web-media-controller/)).

## Usage
wmc-win is a console application that should be started before your browser.
Currently, wmc-win listens for the following keys:

* [`VK_MEDIA_NEXT_TRACK`](https://referencesource.microsoft.com/windowsbase/R/5069862b166d95f1.html)
* [`VK_MEDIA_PREV_TRACK`](https://referencesource.microsoft.com/windowsbase/R/5df9df218d733fe0.html)
* [`VK_MEDIA_STOP`](https://referencesource.microsoft.com/windowsbase/R/ff4207f203a7a13c.html)
* [`VK_MEDIA_PLAY_PAUSE`](https://referencesource.microsoft.com/windowsbase/R/2493a61114ea6df8.html)

## Notes

### Native Messaging
web-media-controller currently uses websockets on Windows, to support the rainmeter plugin.
These are some notes on native messaging, in case a native messaging adapter is added for Windows.

Firefox locates native messaging binaries from the Windows registry, via JSON manifests.
Set the appropriate registry key with the following:

    reg add HKEY_CURRENT_USER\SOFTWARE\Mozilla\NativeMessagingHosts\me.f1u77y.web_media_controller /t REG_SZ /d C:\Users\user\dev\wmc-win\manifests\me.f1u77y.web_media_controller.firefox.json

Make sure the path in the JSON manifest does point to the binary location.
