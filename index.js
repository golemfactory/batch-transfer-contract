var Contract = require('web3-eth-contract');

var fs = require('fs');
var jsonFile = "MultiTransferERC20.abi.json";
var abi = JSON.parse(fs.readFileSync(jsonFile));
var privateKey = fs.readFileSync("private_key.txt");
const provider = new HDWalletProvider(privateKey, 'https://bor.golem.network');

console.log(abi);
console.log(`privateKey: ${privateKey}`);
// set provider for all later instances to use
//Contract.setProvider('http://1.geth.testnet.golem.network:55555');
Contract.setProvider('https://bor.golem.network');

var contract_polygon = new Contract(abi, "0x21cCe3a0F851394fcDD27b28c65232be98fc6Ce2");



contract.methods.GLM().call(function(err, res){
  //do something with res here
  console.log(res); //for example
});
const signPromise = web3.eth.signTransaction(tx, tx.from);

