const { src, dest, series, parallel } = require('gulp');
const replace = require('gulp-replace');

var cp = require('child_process');

function clean(cb) {
  // body omitted
  cb();
}

function ws_launcher_cargo(cb) {
  new Promise((resolve, reject) => {
    cp.exec('cargo web deploy -p ws-launcher', (error) => {
      if (error) { reject(error); }
      resolve();
    });
  }).then(() => { cb(); })
  .catch(error => cb(error));
}

var ws_launcher = series(
  ws_launcher_cargo,
  parallel(
    // js
    function ws_launcher_js(cb) {
      return src('target/deploy/launcher.js')
      // workaround of https://bugzilla.mozilla.org/show_bug.cgi?id=1470182
      .pipe(replace('typeof WebAssembly.instantiateStreaming === "function"', 'false'))
      .pipe(dest('extension/launcher/'))
    },
    // else
    function ws_launcher_others(cb) {
      return src(['target/deploy/launcher.wasm', 'target/deploy/launcher.html'])
      .pipe(dest('extension/launcher/'))
    }
  )
)

function ws_storage_cargo(cb) {
  new Promise((resolve, reject) => {
    cp.exec('cargo web deploy -p ws-storage', (error) => {
      if (error) { reject(error); }
      resolve();
    });
  }).then(() => { cb(); })
  .catch(error => cb(error));
}

var ws_storage = series(
  ws_storage_cargo,
  parallel(
    // js
    function ws_storage_js(cb) {
      return src('target/deploy/storage.js')
      // workaround of https://bugzilla.mozilla.org/show_bug.cgi?id=1470182
      .pipe(replace('typeof WebAssembly.instantiateStreaming === "function"', 'false'))
      .pipe(dest('extension/storage/'))
    },
    // else
    function ws_storage_others(cb) {
      return src(['target/deploy/storage.wasm'])
      .pipe(dest('extension/storage/'))
    }
  )
)


function build(cb) {
  // body omitted
  cb();
}

// exports.build = build;
exports.default = series(clean, parallel(ws_launcher, ws_storage), build);