const path = require("path");

const requireUncached = (module) => {
  delete require.cache[require.resolve(module)];
  return require(module);
};

const requireOrEmpty = (p) => {
  try {
    return requireUncached(path.join(__dirname, p));
  } catch (_e) {
    return {};
  }
};

const getState = () => ({
  ...requireOrEmpty("state.json"),
  ...requireOrEmpty("state.local.json"),
});

module.exports = { getState };
