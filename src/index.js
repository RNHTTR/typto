import * as nearAPI from "near-api-js";

// connect to NEAR
const near = new nearApi.Near({
  keyStore: new nearApi.keyStores.BrowserLocalStorageKeyStore(),
  // networkId: 'local',
  networkId: 'testnet',
  // nodeUrl: 'http://localhost:3030',
  nodeUrl: 'https://rpc.testnet.near.org',
  // walletUrl: 'http://localhost:4000/wallet',
  walletUrl: "https://wallet.testnet.near.org",
  helperUrl: "https://helper.testnet.near.org",
  explorerUrl: "https://explorer.testnet.near.org",
});
  
  // connect to the NEAR Wallet
  const wallet = new nearApi.WalletConnection(near, 'typto');

  // connect to a NEAR smart contract
  const contract = new nearApi.Contract(
    //   document.getElementById('receiver-id').value,
    wallet.account(),
    // 'typto.testnet', 
    'dev-1631209293840-52297054260060',
    {
      changeMethods: ['register_receiver', 'send_tip'],
      sender: wallet.account()
    }
  );

  const button = document.getElementById('sign-in');
  if (wallet.isSignedIn()) {
    console.log("not logged in")
    button.textContent = 'Send tip'
  } else {
    console.log("not logged in")
  }

  // Either sign in or call the addMessage change method on button click
  document.getElementById('sign-in').addEventListener('click', () => {
    if (wallet.isSignedIn()) {
      contract.register_receiver({
        args: { receiver_id: document.getElementById('receiver-id').value },
      })
      console.log("receiver registered")
      console.log(nearApi.utils.format.parseNearAmount(document.getElementById('amount').value))
      contract.send_tip({
        args: {
            receiver_id: document.getElementById('receiver-id').value,
            tip_amount: nearApi.utils.format.parseNearAmount(document.getElementById('amount').value)
            // tip_amount: 100000000000
            // tip_amount: parseInt(document.getElementById('amount').value)
          }
      })
      console.log("funds tipped")
    } else {
      console.log("attempting sign in")
      wallet.requestSignIn({
        // contractId: 'typto.testnet',
        contractId: 'dev-1631209293840-52297054260060',
        methodNames: ['register_receiver', 'send_tip'],
        successUrl: "http://localhost:1234"
      });
    }
  });