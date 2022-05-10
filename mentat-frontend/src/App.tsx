import React, { useEffect } from 'react';
import { useDispatch } from 'react-redux';
import { Outlet } from 'react-router';
import { Content } from './components/Content';
import { SideNav } from './components/SideNav';
import { I18n } from './features/i18n/components';
import { useApi, useNetId } from './features/rosetta/hooks';
import { useNetOptions, useNetStatus } from './features/rosetta/impl';
import { NetworkIdentifier } from './features/rosetta/models';
import {
  setIdentifier,
  setIdentifiers,
  setOptions,
  setStatus,
} from './features/rosetta/reducer';
import { AppDispatch } from './store';

const NetworkHandler: React.FC<{ netId: NetworkIdentifier }> = ({ netId }) => {
  const dispatch: AppDispatch = useDispatch();

  const [, options] = useNetOptions(netId);
  useEffect(() => {
    if (options) dispatch(setOptions(options));
  }, [dispatch, options]);

  const [, status] = useNetStatus(netId);
  useEffect(() => {
    if (status) dispatch(setStatus(status));
  }, [dispatch, status]);

  return null;
};

export const App = () => {
  const dispatch: AppDispatch = useDispatch();

  const [status, list] = useApi<{
    network_identifiers: NetworkIdentifier[];
  }>('/network/list');

  useEffect(() => {
    if (list) {
      dispatch(setIdentifiers(list.network_identifiers));
      if (list.network_identifiers.length === 1)
        dispatch(setIdentifier(list.network_identifiers[0]));
    }
  }, [dispatch, list]);

  const netId = useNetId();

  let message: JSX.Element;
  if (status !== 'ok') {
    if (status !== 'error') message = <I18n name="connection.connecting" />;
    message = <I18n name="connection.invalid" />;
  } else if (typeof list?.network_identifiers?.length !== 'number') {
    message = <I18n name="connection.failed" />;
  }

  return (
    <>
      {netId && <NetworkHandler netId={netId} />}
      <SideNav />
      <Content>{message ?? <Outlet />}</Content>
    </>
  );
};
