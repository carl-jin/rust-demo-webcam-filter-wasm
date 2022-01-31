// Prefer camera resolution nearest to 1280x720.
var constraints = { audio: true, video: { width: 1280, height: 720 } };


setTimeout(() => {
    // navigator.mediaDevices.getUserMedia(constraints)
    navigator.mediaDevices.getUserMedia(constraints)
        .then(function (mediaStream) {
            console.log(mediaStream.getVideoTracks());

            var video = document.querySelector('#videoTarget');
            video.srcObject = mediaStream;
            video.onloadedmetadata = function (e) {
                video.play();
            };
        })
        .catch(function (err) { console.log(err.name + ": " + err.message); });

    // navigator.mediaDevices.enumerateDevices()
    navigator.mediaDevices.enumerateDevices()
        .then(function (devices) {
            // devices.forEach(function (device) {
            //     console.log(device.kind + ": " + device.label +
            //         " id = " + device.deviceId);
            // });
        })
        .catch(function (err) {
            console.log(err.name + ": " + err.message);
        });
}, 1000)