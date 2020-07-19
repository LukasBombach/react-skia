const { spawn } = require("child_process");

const ls = spawn("node", ["lib/openWindow.js"]);

ls.stdout.on("data", (data) => {
  console.log(`stdout: ${data}`);
});

ls.stderr.on("data", (data) => {
  console.error(`stderr: ${data}`);
});

ls.on("close", (code) => {
  console.log(`child process exited with code ${code}`);
});

console.log("hello world");

setTimeout(() => {
  console.log("hello again");
}, 2000);

setTimeout(() => {
  console.log("and again");

  setInterval(() => {
    console.log("round and round we go");
  }, 2000);
}, 3000);

console.log("yadoodle");
