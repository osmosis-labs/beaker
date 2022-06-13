const path = require("path");

const requireOrEmpty = (p) => {
  try {
    return require(path.join(__dirname, p));
  } catch (_e) {
    return {};
  }
};

const getState = () => ({
  ...requireOrEmpty("state.json"),
  ...requireOrEmpty("state.local.json"),
});

module.exports = { getState };
