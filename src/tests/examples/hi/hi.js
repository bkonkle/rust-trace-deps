const one = require("one");
require("two");
require(`three`);

const variableDep = "missing-dynamic-pkg";
require(variableDep);
require.resolve("missing-static-pkg");
