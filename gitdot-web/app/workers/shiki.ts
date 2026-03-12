/// <reference lib="webworker" />

self.onmessage = function (event) {
  console.log("shiki worker received:", event.data);
};
