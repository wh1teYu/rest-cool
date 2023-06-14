import test from "ava";

import { concatStr, sum, getOptions } from "../index.js";

test("sum from native", (t) => {
  t.is(sum(1, 2), 3);
});

// 增加测试
test("concatStr from native", (t) => {
  t.is(concatStr("Hello", "World"), "HelloWorld");
});

test("getOptions from native", (t) => {
  const options = {
    id: 1,
    name: "napi-rs",
  };
  t.deepEqual(getOptions(options), options);
});
