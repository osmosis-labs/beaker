const path = require('path')

const getState = (mode) => {
	switch (mode) {
		case "local":
			return require(path.join(__dirname, 'state.local.json'))
		default:
			return require(path.join(__dirname, 'state.json'))
	}
}


module.exports = { getState };
