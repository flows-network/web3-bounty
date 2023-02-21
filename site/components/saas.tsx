import { useState, useEffect, useRef } from 'react';
import { Container, Row, Col, Stack, Image, Button } from 'react-bootstrap';
import { SaaS } from '../types/saas';
import { getConnectedSaaS } from '../lib/flows_api';

const All_SAAS = [
  {
    name: 'GitHub',
    icon: 'https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png',
    icon_width: 45,
    oauth: 'https://github.com/apps/web3bountydemo/installations/new?state={account}',
    username_field: 'Login',
    connected: null as any
  },
  {
    name: 'Twitter',
    icon: 'https://www.wellybox.com/wp-content/uploads/2023/02/pngkey.com-twitter-logo-png-transparent-27646.png',
    icon_width: 33,
    oauth: `${process.env.NEXT_PUBLIC_TWITTER_PRE_AUTH_PATH}?account={account}`,
    username_field: 'Username',
    connected: null as any
  }
];

export default function SaaSList({account, showAlert}: any) {
  const [connecting, setConnecting] = useState(null as string | null);
  const connectingRef = useRef(connecting);
  connectingRef.current = connecting;

  const [saas, setSaaS] = useState(null as Array<SaaS> | null);

  function connect(name: string) {
    setConnecting(name);
    keepCheck([name]);
  }

  async function keepCheck(sa: Array<string>) {
    setTimeout(async () => {
      await check(sa);
      if (connectingRef.current) {
        keepCheck(sa);
      }
    }, 1000);
  }

  async function check(sa: Array<string>) {
    try {
      let s = await getConnectedSaaS(account, sa);

      if (s) {
        let connected = s.find((a: any) => {
          return a.name == connectingRef.current;
        });
        if (connected) {
          setConnecting(null);
        }
      }

      setSaaS(s);
    } catch (e: any) {
      showAlert({
        variant: 'danger',
        message: e.toString()
      });
    }
  }

  useEffect(() => {
    account && check(All_SAAS.map(s => s.name));
  }, [account]);

  if (saas) {
    return (
      <Stack gap={4} className="col-md-5 mx-auto my-5">
        {
          All_SAAS.map((s) => {
            s.connected = s.connected || saas.find((a: any) => a.name === s.name);
            return (
              <Container key={s.name} className={`${s.connected ? 'bg-success border-success' : 'bg-secondary border-secondary' } border border-opacity-25 bg-gradient bg-opacity-25 rounded p-2`}>
                <Row className="align-items-center">
                  <Col xs={2}>
                    <div className="d-flex align-items-center justify-content-center border border-1 rounded-2 overflow-hidden bg-white" style={{width: 50, height: 50}}>
                      <Image width={s.icon_width} src={s.icon} />
                    </div>
                  </Col>
                  <Col xs={8} style={{fontSize: '0.875em'}}>
                    {
                      s.connected
                      ? (
                        <span>
                        Connected to <em>{(s.connected as any).fields[s.username_field]}</em>
                        </span>
                      )
                      : `Please connect with your ${s.name} account`
                    }
                  </Col>
                  <Col xs={2} className="text-center">
                    {
                      s.connected
                      ? (
                        <i className="fs-4 bi bi-shield-check"></i>
                      )
                      : (
                        <Button onClick={() => {connect(s.name);} } href={s.oauth.replace('{account}', account)} target="_blank" variant="primary" size="sm" className="rounded-circle" disabled={connecting != null}>
                        {
                          connecting == s.name
                          ? <span className="spinner-border spinner-border-sm" role="status">
</span>
                          : <i className="bi bi-plus-lg"></i>
                        }
                        </Button>
                      )
                    }
                  </Col>
                </Row>
              </Container>
            );
          })
        }
      </Stack>
    );
  }

  return null;
}

