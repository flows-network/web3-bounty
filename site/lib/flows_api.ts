import {SaaS} from '../types/saas';

export async function getConnectedSaaS(account: string): Promise<Array<SaaS>> {
  const response = await fetch(
    `https://code.flows.network/lambda/dElmOPdyu2?account=${account}`
  )
  const data: Array<any> = await response.json(); 
  if (data && data.length > 0) {
    return [{
      name: 'github',
      fields: data[0].fields
    }];
  } else {
    return [];
  }
}

