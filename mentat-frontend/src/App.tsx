import * as React from 'react';
import { Outlet } from 'react-router';
import { Content } from './components/Content';
import { SideNav } from './components/SideNav';
import { I18n } from './features/i18n/components';
import { useApi } from './features/rosetta/hooks';
import { NetworkIdentifier } from './features/rosetta/models';

export const NetworkIdentContext = React.createContext<NetworkIdentifier>(null);

export const useNetId = () => React.useContext(NetworkIdentContext);

export const App = () => {
  const [status, resp] = useApi<{
    network_identifiers: NetworkIdentifier[];
  }>('/network/list');

  let message: JSX.Element;
  if (status !== 'ok') {
    if (status !== 'error') message = <I18n name="connection.connecting" />;
    message = <I18n name="connection.invalid" />;
  } else if (typeof resp?.network_identifiers?.length !== 'number') {
    message = <I18n name="connection.failed" />;
  }

  return (
    <>
      <NetworkIdentContext.Provider
        value={resp?.network_identifiers?.[0] ?? null}
      >
        <SideNav />
        <Content>{message ?? <Outlet />}</Content>
      </NetworkIdentContext.Provider>
    </>
  );
};
