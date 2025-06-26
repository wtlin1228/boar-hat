// preview-server.js
import sirv from "sirv";
import { createServer } from "http";
import { resolve } from "path";

const serve = sirv(resolve("dist"), {
  gzip: false,
  brotli: false,
  single: true,
});

const server = createServer((req, res) => {
  serve(req, res);
});

server.listen(4173, () => {
  console.log("Preview server running at http://localhost:4173");
});
