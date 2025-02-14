import { execute } from "./lib.ts";

async function main() {
  let ret = await execute("world");
  console.log(ret);
}

export default main;
