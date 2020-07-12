var { Canvas } = require("../native");

function sleep(n) {
  return new Promise((resolve) => setTimeout(resolve, n));
}

async function run() {
  const canvas = new Canvas();
  console.log("post canvas");

  await sleep(1000);
  console.log("post sleep");

  canvas.open();
  console.log("post open");
}

run();
