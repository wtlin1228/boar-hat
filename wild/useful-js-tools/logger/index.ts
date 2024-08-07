import { logger, enableDebug } from "./logger";

enableDebug();

const a1 = logger.extend("a1");
const a1b1 = a1.extend("b1");
const a1b2 = a1.extend("b2");

const a2 = logger.extend("a2");
const a2b1 = a2.extend("b1");
const a2b2 = a2.extend("b2");

a1("hello world"); // 🟥 root:a1 hello world +0ms
a1b1("hello world"); // 🟧 root:a1:b1 hello world +0ms
a1b2("hello world"); // 🟨 root:a1:b2 hello world +0ms
a2("hello world"); // 🟩 root:a2 hello world +0ms
a2b1("hello world"); // 🟦 root:a2:b1 hello world +0ms
a2b2("hello world"); // 🟪 root:a2:b2 hello world +0ms
