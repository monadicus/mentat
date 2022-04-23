import * as React from 'react';
import { Outlet } from 'react-router';
import { Content } from './components/Content';
import { SideNav } from './components/SideNav';
import { I18n } from './features/i18n/components';
import { useApi } from './features/rosetta/hooks';

export const App = () => {
  const [status, resp] = useApi<{ network_identifiers: unknown[] }>(
    '/network/list'
  );

  let message: JSX.Element;
  if (status !== 'ok') {
    if (status !== 'error') message = <I18n name="connection.connecting" />;
    message = <I18n name="connection.invalid" />;
  } else if (typeof resp?.network_identifiers?.length !== 'number') {
    message = <I18n name="connection.failed" />;
  }

  return (
    <>
      <SideNav />
      <Content>{message ?? <Outlet />}</Content>
    </>
  );
};
