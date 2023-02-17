import { NextPage } from 'next';
import { useEffect } from 'react';

const ConnectedPage: NextPage = () => {
  useEffect(() => {
    (window as any).close();
  });

  return (
    null
  );
};

export default ConnectedPage
