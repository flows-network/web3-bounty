import {SaaS} from '../types/saas';

export async function getConnectedSaaS(account: string, tables: Array<string>): Promise<Array<SaaS>> {
  let result: Array<SaaS> = [];

  for (let i = 0; i < tables.length; i++) {
    let t = tables[i];
    const response = await fetch(
      `${process.env.NEXT_PUBLIC_SEARCH_PATH}?account=${account}&table=${t}`
    )
    if (!response.ok) {
      throw `Can not search data for '${t}'`;
    }
    const data: Array<any> = await response.json(); 
    if (data && data.length > 0) {
      result.push({
        name: t.toLowerCase(),
        fields: data[0].fields
      });
    }
  }

  return result;
}

