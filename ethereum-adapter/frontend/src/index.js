/*****************************************/
/* Detect the MetaMask Ethereum provider */
/*****************************************/

import detectEthereumProvider from '@metamask/detect-provider';

const provider = await detectEthereumProvider();

if (provider) {
  startApp(provider);
} else {
  console.log('Please install MetaMask!');
}

function startApp(provider) {
  if (provider !== window.ethereum) {
    console.error('Do you have multiple wallets installed?');
  }
}

/**********************************************************/
/* Handle chain (network) and chainChanged (per EIP-1193) */
/**********************************************************/

const chainId = await window.ethereum.request({ method: 'eth_chainId' });
let metamaskAccounts = [];

window.ethereum.on('chainChanged', handleChainChanged);

function handleChainChanged(chainId) {
  window.location.reload();
}

/***********************************************************/
/* Handle user accounts and accountsChanged (per EIP-1193) */
/***********************************************************/

let currentAccount = null;
window.ethereum.request({ method: 'eth_accounts' })
  .then(handleAccountsChanged)
  .catch((err) => {
    console.error(err);
  });

window.ethereum.on('accountsChanged', handleAccountsChanged);

function handleAccountsChanged(accounts) {
  metamaskAccounts = accounts;
  if (accounts.length === 0) {
    connectButton.classList.remove('hidden')
    connected.classList.add('hidden')
  } else if (accounts[0] !== currentAccount) {
    currentAccount = accounts[0];
    storeAccount(currentAccount);
  }
}

/*********************************************/
/* Access the user's accounts (per EIP-1102) */
/*********************************************/

const connectButton = document.querySelector('.connectButton');
const showAccount = document.querySelector('.showAccount');
const connected = document.querySelector('#connected');

connectButton.addEventListener('click', () => {
  getAccount();
});

async function getAccount() {
  const accounts = await window.ethereum.request(
    { method: 'eth_requestAccounts' })
    .catch((err) => {
      if (err.code === 4001) {
        console.log('Please connect to MetaMask.');
      } else {
        // console.error(err);
      }
    });
  metamaskAccounts = accounts;
  const account = accounts[0];
  storeAccount(account);
  return account;
}

function storeAccount(account) {
  connectButton.classList.add('hidden')
  showAccount.innerHTML = account;
  connected.classList.remove('hidden')
  fetch('/address', {
      method: 'POST',
      headers: {
          'Accept': 'application/json',
          'Content-Type': 'application/json'
      },
      body: JSON.stringify({ "address": account })
  });
}

/*********************************************/
/* Sign Deployment */
/*********************************************/
const signButton = document.querySelector(".signButton")
const signed = document.querySelector('#signed')
const signature = document.querySelector('.signature')

if (signButton) {
  signButton.addEventListener('click', () => {
    signPayload();
  })
}

const toHexString = (bytes) => {
  return Array.from(bytes, (byte) => {
    return ('0' + (byte & 0xff).toString(16)).slice(-2);
  }).join('');
};

function storeSignature(account, signature) {
  fetch('/signature', {
      method: 'POST',
      headers: {
          'Accept': 'application/json',
          'Content-Type': 'application/json'
      },
      body: JSON.stringify({ by: { "address": account }, "signature": signature })
  });
}


async function signPayload() {
  const urlParams = new URLSearchParams(window.location.search);
  const payload = urlParams.get('payload');

  try {
    const from = metamaskAccounts[0];

    let enc = new TextEncoder(); 
    const msg = `0x${toHexString(enc.encode(payload))}`;

    const sign = await ethereum.request({
      method: 'personal_sign',
      params: [msg, from],
    });

    storeSignature(from, sign);
    signed.classList.remove('hidden');
  } catch (err) {
    console.error(err);
  }
}