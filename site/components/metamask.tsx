import { Badge, Button } from 'react-bootstrap';
import { useState, useEffect } from 'react';

export default function MetaMask({done, showAlert}: any) {
  const [metamaskReady, setMetamaskReady] = useState(false);
  const [pending, setPending] = useState(false);
  const [connected, setConnected] = useState(false);

  useEffect(() => {
    if (typeof (window as any).ethereum !== 'undefined' &&
      (window as any).ethereum.isMetaMask) {
      setMetamaskReady(true);
    }
  });

  async function connect() {
    setPending(true);
    try {
      const accounts: string[] = await (window as any).ethereum.request({ method: 'eth_requestAccounts' });
      if (accounts && accounts.length > 0) {
        done(accounts);
        setConnected(true);
      } else {
        throw `Can't get accounts`;
      }
    } catch(e) {
      showAlert({
        variant: 'danger',
        message: (e as any).toString(),
      });
    } finally {
      setPending(false);
    }
  }
  
  return (
    connected
    ?
    <div className="text-center">
      <Badge bg="success">Connected with MetaMask</Badge>
    </div>
    : (
      metamaskReady
      ? <Button onClick={connect} variant="primary" disabled={pending}>Connect with MetaMask</Button>
      : <Button variant="outline-secondary" disabled>MetaMask is not installed</Button>
      )
  );
}

