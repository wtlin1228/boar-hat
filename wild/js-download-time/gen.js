// generate-large-js.mjs
import { writeFile } from "fs/promises";

let content = "// Large JS file for testing\n\n";

content += "const data = [];";

for (let i = 0; i < 100000; i++) {
  content += `
function testFunction_${i}() {
  return "This is test function number ${i}";
}
`;
}

for (let i = 0; i < 50000; i++) {
  content += `
if (window.somevar_${i}) {
  data.push(testFunction_${i}());
} else {
  data.push(testFunction_${i * 2}());
}
`;
}

content += "const result = data.join(',,,,--,,,,');";
content += "console.log(result);";

// Write the file
await writeFile(
  "./public/large.js",
  `
(function() {
  ${content}
})()    
`
);
