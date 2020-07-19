// index.js
// run with node --experimental-worker index.js on Node.js 10.x
const { Worker } = require("worker_threads");

function runService() {
  return new Promise((resolve, reject) => {
    const worker = new Worker("./lib/openWindow.js");
    worker.on("message", resolve);
    worker.on("error", reject);
    worker.on("exit", (code) => {
      if (code !== 0)
        reject(new Error(`Worker stopped with exit code ${code}`));
    });
  });
}

async function run() {
  const result = await runService();
  console.log(result);
}

run().catch((err) => console.error(err));

/* const {
  Worker,
  isMainThread,
  parentPort,
  workerData,
} = require("worker_threads");

function openWindow(script) {
  return new Promise((resolve, reject) => {
    const worker = new Worker(__filename, {
      workerData: script,
    });
    worker.on("message", resolve);
    worker.on("error", reject);
    worker.on("exit", (code) => {
      if (code !== 0)
        reject(new Error(`Worker stopped with exit code ${code}`));
    });
  });
}

if (isMainThread) {
} else {
  const { parse } = require("some-js-parsing-library");
  const script = workerData;
  parentPort.postMessage(parse(script));
}
 */
