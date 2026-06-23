# VDL Browser Extension Guide

## Method 1: Native Messaging (Recommended)

### 1. Install VDL Binary
```bash
# Build binary
cargo build --release

# Install to system path
sudo cp target/release/vdl /usr/local/bin/vdl
```

### 2. Create Native Messaging Host

**Linux:** `~/.mozilla/native-messaging-hosts/vdl.json`
```json
{
  "name": "vdl",
  "description": "VDL Video Downloader",
  "path": "/usr/local/bin/vdl",
  "type": "stdio",
  "allowed_origins": [
    "chrome-extension://[EXTENSION-ID]/"
  ]
}
```

**Windows:** Registry entry at `HKEY_CURRENT_USER\Software\Mozilla\NativeMessagingHosts\vdl`

### 3. Browser Extension Manifest

```json
{
  "manifest_version": 3,
  "name": "VDL Downloader",
  "version": "1.0",
  "permissions": ["contextMenus"],
  "background": {
    "service_worker": "background.js"
  },
  "action": {
    "default_popup": "popup.html"
  }
}
```

### 4. Background Script

```javascript
// background.js
chrome.contextMenus.create({
  id: "vdl-download",
  title: "Download with VDL",
  contexts: ["link"]
});

chrome.contextMenus.onClicked.addListener((info, tab) => {
  if (info.menuItemId === "vdl-download") {
    downloadVideo(info.linkUrl);
  }
});

function downloadVideo(url) {
  // Use native messaging or execute script
  chrome.scripting.executeScript({
    target: {tabId: tab.id},
    func: triggerDownload,
    args: [url]
  });
}
```

## Method 2: Bookmarklet

```javascript
javascript:(function(){
  const url = window.location.href;
  window.open('http://localhost:8080/download/' + encodeURIComponent(url));
})();
```

## Method 3: Userscript (Tampermonkey)

```javascript
// ==UserScript==
// @name         VDL Downloader
// @match        *://*/*
// @grant        GM_openInTab
// ==/UserScript==

(function() {
  'use strict';
  const button = document.createElement('button');
  button.textContent = '📥 Download';
  button.onclick = () => GM_openInTab('/download/' + location.href);
  document.body.appendChild(button);
})();
```

## Quick Start

1. Build VDL: `cargo build --release`
2. Install native host (Linux example):
   ```bash
   mkdir -p ~/.mozilla/native-messaging-hosts
   cp vdl.json ~/.mozilla/native-messaging-hosts/
   ```
3. Load extension in browser
4. Right-click any video link → "Download with VDL"

**No server required!** 🎉
