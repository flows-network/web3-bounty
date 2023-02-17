import 'bootstrap/dist/css/bootstrap.css';
import 'bootstrap-icons/font/bootstrap-icons.css';
import Head from 'next/head';
import { NextPage } from 'next';
import { useState } from 'react';
import MetaMask from '../components/metamask';
import Alert from '../components/alert';
import SaaSList from '../components/saas';
import { AlertParams } from '../types/alert';

import { Container, Row, Col, Stack } from 'react-bootstrap';

const IndexPage: NextPage = () => {
  const [accounts, setAccounts] = useState([] as string[]);
  const [alertShow, setAlertShow] = useState(false);
  const [alertParams, setAlertParams] = useState(null as AlertParams | null);

  async function metamaskConnected(accounts: string[]) {
    setAccounts(accounts);
  }

  function showAlert(params: AlertParams) {
    setAlertParams(params);
    setAlertShow(true);
  }

  function hideAlert() {
    setAlertShow(false);
  }

  return (
    <main>
      <Head>
        <title>web3-bounty</title>
        <meta name="viewport" content="width=device-width, initial-scale=1"/>
      </Head>

      <Container>
        <Row className="align-items-center position-relative" style={{minHeight: '100vh'}}>

          {alertShow && <Alert params={alertParams} hide={hideAlert} />}

          <Col>
            <Container style={{paddingBottom: '5em'}}>
              <Stack gap={2} className="col-md-5 mx-auto">
                <h1 className="my-3 fw-bold text-center">Web3 Bounty</h1>
                <MetaMask done={metamaskConnected} showAlert={showAlert} />
              </Stack>

              <SaaSList account={accounts[0]} showAlert={showAlert} />
            </Container>
          </Col>
        </Row>

      </Container>

    </main>
  )
}

export default IndexPage
