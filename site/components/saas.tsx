import { useState, useEffect, useRef } from 'react';
import { Container, Row, Col, Stack, Image, Button } from 'react-bootstrap';
import { SaaS } from '../types/saas';
import { getConnectedSaaS } from '../lib/flows_api';

const All_SAAS = [
  {
    name: 'github',
    label: 'GitHub',
    icon: 'https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png',
    oauth: 'https://github.com/apps/web3bountydemo/installations/new?state={account}',
    username_field: 'Login'
  }
];

export default function SaaSList({account, showAlert}: any) {
  const [connecting, setConnecting] = useState(null as string | null);
  const connectingRef = useRef(connecting);
  connectingRef.current = connecting;

  const [saas, setSaaS] = useState(null as Array<SaaS> | null);
  const saasRef = useRef(saas);
  saasRef.current = saas;

  function connect(name: string) {
    setConnecting(name);
    keepCheck();
  }

  async function keepCheck() {
    setTimeout(async () => {
      await check();
      if (connectingRef.current) {
        keepCheck();
      }
    }, 1000);
  }

  async function check() {
    try {
      setSaaS(await getConnectedSaaS(account));
      if (saasRef.current) {
        let connected = saasRef.current.find((a: any) => {
          a.name === connectingRef.current
        });
        if (connected) {
          setConnecting(null);
        }
      }
    } catch (e: any) {
      showAlert({
        variant: 'danger',
        message: e.toString()
      });
    }
  }

  useEffect(() => {
    account && check();
  }, [account]);

  if (saas) {
    return (
      <Stack gap={4} className="col-md-5 mx-auto my-5">
        {
          All_SAAS.map((s) => {
            let connected = saas.find((a: any) => a.name === s.name);
            return (
              <Container key={s.name} className={`${connected ? 'bg-success border-success' : 'bg-secondary border-secondary' } border border-opacity-25 bg-gradient bg-opacity-25 rounded p-2`}>
                <Row className="align-items-center">
                  <Col sm={2}>
                    <Image width={50} src={s.icon} thumbnail fluid />
                  </Col>
                  <Col sm={8} style={{fontSize: '0.875em'}}>
                    {
                      connected
                      ? (
                        <span>
                        Connected to <em>{(connected as any).fields[s.username_field]}</em>
                        </span>
                      )
                      : `Please connect with your ${s.label} account`
                    }
                  </Col>
                  <Col sm={2} className="text-center">
                    {
                      connected
                      ? (
                        <i className="fs-4 bi bi-shield-check"></i>
                      )
                      : (
                        <Button onClick={() => {connect(s.name);} } href={s.oauth.replace('{account}', account)} target="_blank" variant="primary" size="sm" className="rounded-circle" disabled={connecting != null}>
                        {
                          connecting
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

