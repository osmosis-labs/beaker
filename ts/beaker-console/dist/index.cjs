/*!
 * beaker-console v0.1.5
 * (c) Supanat Potiwarakorn
 * Released under the MIT OR Apache-2.0 License.
 */

'use strict';

Object.defineProperty(exports, '__esModule', { value: true });

var crypto = require('@cosmjs/crypto');
var cosmwasm = require('cosmwasm');
var _case = require('case');

/******************************************************************************
Copyright (c) Microsoft Corporation.

Permission to use, copy, modify, and/or distribute this software for any
purpose with or without fee is hereby granted.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
PERFORMANCE OF THIS SOFTWARE.
***************************************************************************** */

var __assign = function() {
    __assign = Object.assign || function __assign(t) {
        for (var s, i = 1, n = arguments.length; i < n; i++) {
            s = arguments[i];
            for (var p in s) if (Object.prototype.hasOwnProperty.call(s, p)) t[p] = s[p];
        }
        return t;
    };
    return __assign.apply(this, arguments);
};

function __awaiter(thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
}

function __generator(thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (_) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
}

/**
 * Account instance with baked-in client and utility methods
 */
var Account = /** @class */ (function () {
    function Account(wallet, signingClient, address) {
        this.wallet = wallet;
        this.signingClient = signingClient;
        this.address = address;
    }
    Account.withDerivedAddress = function (wallet, signingClient) {
        return __awaiter(this, void 0, void 0, function () {
            var accountData;
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0: return [4 /*yield*/, wallet.getAccounts()];
                    case 1:
                        accountData = (_a.sent())[0];
                        if (accountData === undefined) {
                            throw Error('address not found');
                        }
                        return [2 /*return*/, new Account(wallet, signingClient, accountData.address)];
                }
            });
        });
    };
    /**
     * Get balances for specific denom, only support native coin
     */
    Account.prototype.getBalance = function (denom) {
        var _a;
        return __awaiter(this, void 0, void 0, function () {
            var accounts, address;
            return __generator(this, function (_b) {
                switch (_b.label) {
                    case 0: return [4 /*yield*/, this.wallet.getAccounts()];
                    case 1:
                        accounts = _b.sent();
                        address = (_a = accounts[0]) === null || _a === void 0 ? void 0 : _a.address;
                        if (!address) {
                            throw Error("No account not found from: ".concat(accounts));
                        }
                        return [4 /*yield*/, this.signingClient.getBalance(address, denom)];
                    case 2: return [2 /*return*/, _b.sent()];
                }
            });
        });
    };
    return Account;
}());
var fromMnemonic = function (conf, network, mnemonic) { return __awaiter(void 0, void 0, void 0, function () {
    var options, wallet, networkInfo, signingClient;
    return __generator(this, function (_a) {
        switch (_a.label) {
            case 0:
                if (typeof conf.global.account_prefix !== 'string') {
                    throw Error('`account_prefix` must be string');
                }
                options = {
                    prefix: conf.global.account_prefix,
                    hdPaths: [crypto.stringToPath(conf.global.derivation_path)],
                };
                return [4 /*yield*/, cosmwasm.Secp256k1HdWallet.fromMnemonic(mnemonic, options)];
            case 1:
                wallet = _a.sent();
                networkInfo = conf.global.networks[network];
                if (!networkInfo) {
                    throw Error("network info for ".concat(network, " not found in the config"));
                }
                return [4 /*yield*/, cosmwasm.SigningCosmWasmClient.connectWithSigner(networkInfo.rpc_endpoint, wallet, { gasPrice: cosmwasm.GasPrice.fromString(conf.global.gas_price) })];
            case 2:
                signingClient = _a.sent();
                return [2 /*return*/, Account.withDerivedAddress(wallet, signingClient)];
        }
    });
}); };
var getAccounts = function (conf, network) { return __awaiter(void 0, void 0, void 0, function () {
    var accountName, account;
    return __generator(this, function (_a) {
        switch (_a.label) {
            case 0:
                accountName = Object.keys(conf.global.accounts);
                return [4 /*yield*/, Promise.all(Object.values(conf.global.accounts).map(function (a) {
                        return fromMnemonic(conf, network, a.mnemonic);
                    }))];
            case 1:
                account = _a.sent();
                return [2 /*return*/, Object.fromEntries(accountName.map(function (name, i) { return [name, account[i]]; }))];
        }
    });
}); };

/* eslint-disable */
var mapObject = function (o, f, g) {
    return Object.fromEntries(Object.entries(o).map(function (_a) {
        var k = _a[0], v = _a[1];
        return [f(k), g(v)];
    }));
};
var mapKV = function (o, f) {
    return Object.fromEntries(Object.entries(o).map(function (_a) {
        var k = _a[0], v = _a[1];
        return f(k, v);
    }));
};
var extendWith = function (properties) {
    return function (context) {
        Object.entries(properties).forEach(function (_a) {
            var k = _a[0], v = _a[1];
            // @ts-ignore
            context[k] = v;
            // Object.defineProperty(context, k, {
            //   configurable: true,
            //   enumerable: true,
            //   value: v,
            // });
        });
    };
};

/**
 * Contract instance with baked-in client
 */
var Contract = /** @class */ (function () {
    function Contract(address, client) {
        this.address = address;
        this.client = client;
    }
    /**
     * Get contract info
     */
    Contract.prototype.getInfo = function () {
        return __awaiter(this, void 0, void 0, function () {
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0: return [4 /*yield*/, this.client.getContract(this.address)];
                    case 1: return [2 /*return*/, _a.sent()];
                }
            });
        });
    };
    /**
     * Get code details
     */
    Contract.prototype.getCode = function () {
        return __awaiter(this, void 0, void 0, function () {
            var _a, _b;
            return __generator(this, function (_c) {
                switch (_c.label) {
                    case 0:
                        _b = (_a = this.client).getCodeDetails;
                        return [4 /*yield*/, this.getInfo()];
                    case 1: return [2 /*return*/, _b.apply(_a, [(_c.sent()).codeId])];
                }
            });
        });
    };
    /**
     * Query the contract by passing query message
     * @returns query result
     */
    Contract.prototype.query = function (qmsg) {
        return __awaiter(this, void 0, void 0, function () {
            return __generator(this, function (_a) {
                return [2 /*return*/, this.client.queryContractSmart(this.address, qmsg)];
            });
        });
    };
    /**
     * Execute the contract.
     * example usage: `contract.execute(xmsg).by(signerAccount)`
     */
    Contract.prototype.execute = function (xmsg, senderAddress, fee) {
        var _this = this;
        if (fee === void 0) { fee = 'auto'; }
        return {
            by: function (account) {
                return executor(account, _this.address)(xmsg, senderAddress, fee);
            },
        };
    };
    return Contract;
}());
var getContracts = function (client, state, 
/* eslint-disable */
// @ts-ignore
sdk) {
    return mapKV(state, function (contractName, contractInfo) {
        var addresses = contractInfo.addresses;
        var prefixLabel = function (label) { return "$".concat(label); };
        var pascalContractName = _case.pascal(contractName);
        var contractSdk = errorIfNotFound(sdk.contracts[pascalContractName], "\"".concat(pascalContractName, "\" not found in sdk."));
        var contractQueryClient = warnIfNotFound(contractSdk["".concat(pascalContractName, "QueryClient")], "\"".concat(pascalContractName, "QueryClient\" not found in \"").concat(contractName, "\" contract's sdk. This may caused by empty QueryMsg variant."));
        var contractClient = warnIfNotFound(contractSdk["".concat(pascalContractName, "Client")], "\"".concat(pascalContractName, "Client\" not found in \"").concat(contractName, "\" contract's sdk. This may caused by empty ExecuteMsg variant."), true);
        var contracts = mapObject(addresses, prefixLabel, 
        // (addr: string) => ,
        function (addr) { return (__assign(__assign(__assign({}, new Contract(addr, client)), evalOrElse(contractQueryClient, function (cqc) { return new cqc(client, addr); }, {})), { signer: function (account) {
                return __assign(__assign({}, evalOrElse(contractClient, function (cq) {
                    return new cq(account.signingClient, account.address, addr);
                }, {})), { execute: executor(account, addr) });
            } })); });
        if (typeof contracts['$default'] === 'object' && contracts['$default']) {
            contracts = __assign(__assign({}, contracts), contracts['$default']);
            Object.setPrototypeOf(contracts, Contract.prototype);
        }
        return [contractName, contracts];
    });
};
var executor = function (account, contractAddress) {
    return function (msg, senderAddress, fee) {
        if (fee === void 0) { fee = 'auto'; }
        return __awaiter(void 0, void 0, void 0, function () {
            var _senderAddress, _a;
            var _b;
            return __generator(this, function (_c) {
                switch (_c.label) {
                    case 0:
                        _a = senderAddress;
                        if (_a) return [3 /*break*/, 2];
                        return [4 /*yield*/, account.wallet.getAccounts()];
                    case 1:
                        _a = ((_b = (_c.sent())[0]) === null || _b === void 0 ? void 0 : _b.address);
                        _c.label = 2;
                    case 2:
                        _senderAddress = _a;
                        if (!_senderAddress) {
                            throw Error('Unable to get sender address');
                        }
                        return [4 /*yield*/, account.signingClient.execute(_senderAddress, contractAddress, msg, fee)];
                    case 3: return [2 /*return*/, _c.sent()];
                }
            });
        });
    };
};
var errorIfNotFound = function (object, msg) {
    if (object === undefined) {
        throw Error(msg);
    }
    else {
        return object;
    }
};
var warnIfNotFound = function (object, msg, last) {
    if (last === void 0) { last = false; }
    if (object === undefined) {
        process.stdout.clearLine(0, function () {
            process.stdout.cursorTo(0, function () {
                console.log('\u001B[33m[WARN] ' + msg);
                if (last) {
                    process.stdout.emit('resize'); // a hack to get prompt back after all warnings
                }
            });
        });
        return object;
    }
    else {
        return object;
    }
};
var evalOrElse = function (object, f, orElse) {
    if (object !== undefined) {
        return f(object);
    }
    else {
        return orElse;
    }
};

exports.Account = Account;
exports.Contract = Contract;
exports.extendWith = extendWith;
exports.getAccounts = getAccounts;
exports.getContracts = getContracts;
//# sourceMappingURL=index.cjs.map
