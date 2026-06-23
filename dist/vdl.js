// VDL JavaScript Wrapper
// For browser extension use

class VDL {
    constructor() {
        this.binaryPath = '/usr/local/bin/vdl';
    }

    async download(url, options = {}) {
        // For browser extension: use native messaging
        return new Promise((resolve, reject) => {
            const port = chrome.runtime.connectNative('vdl');
            
            port.onMessage.addListener((response) => {
                if (response.error) {
                    reject(new Error(response.error));
                } else {
                    resolve(response);
                }
            });

            port.postMessage({
                action: 'download',
                url: url,
                output: options.output || 'video.mp4'
            });
        });
    }
}

window.VDL = new VDL();
