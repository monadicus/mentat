import { nanoid } from '@reduxjs/toolkit';
import React, { MouseEventHandler, useCallback, useMemo } from 'react';
import { FaPlus } from 'react-icons/fa';
import { useDispatch, useSelector } from 'react-redux';
import { useNavigate, useParams } from 'react-router';
import styled from 'styled-components';
import { netIdStr } from '../../features/accounts/util';
import { addError } from '../../features/errors/reducer';
import { I18n } from '../../features/i18n/components';
import { findCrypto, getCryptoIcon } from '../../features/icons/crypto';
import { setMentatStatus } from '../../features/mentat/reducer';
import {
  selectHasBackend,
  selectMentatServers,
} from '../../features/mentat/selectors';
import {
  isRosettaError,
  useEndpointUrl,
  useNetId,
} from '../../features/rosetta/hooks';
import { resetRosetta } from '../../features/rosetta/reducer';
import { AppDispatch } from '../../store';

export const NetworkNavStyle = styled.nav`
  height: ${props => props.theme.margin * 3}px !important;
  background-color: ${props => props.theme.colors.main};

  display: flex;

  .network {
    align-self: stretch;
    display: flex;
    align-items: center;
    padding: 8px;
    border-bottom: 2px solid transparent;
    cursor: pointer;
    &:hover {
      background-color: ${props => props.theme.colors.accentHalf};
      border-bottom-color: ${props => props.theme.colors.accent};
    }
    &.active {
      border-bottom-color: ${props => props.theme.colors.accent};
    }

    &.button {
      gap: 4px;
    }

    .crypto-icon {
      width: ${props => props.theme.margin * 2}px;
      height: ${props => props.theme.margin * 2}px;
      margin-right: 4px;
      object-fit: contain;
    }
  }
`;

export const NetworkNav = () => {
  const hasBackend = useSelector(selectHasBackend);
  const servers = useSelector(selectMentatServers);
  const navigate = useNavigate();
  const { endpoint } = useParams();
  const dispatch: AppDispatch = useDispatch();

  // current network information
  const url = useEndpointUrl();
  const netId = useNetId();

  const networks = useMemo(() => Object.keys(servers), [servers]);

  const onNetworkClicked =
    (id: string): MouseEventHandler<HTMLDivElement> =>
    event => {
      event.preventDefault();
      dispatch(resetRosetta());
      navigate(`/${id}/`);
    };

  const onAddNetworkClicked = useCallback(async () => {
    try {
      const crypto = findCrypto(netId?.blockchain);

      const id =
        [crypto.symbol, netIdStr(netId)]
          .filter(Boolean)
          .find(name => name in servers) ?? nanoid();

      const name = netId?.blockchain || crypto.name;

      if (!name) return;

      const resp = await fetch(`/api/v1/servers/${id}`, {
        method: 'POST',
        headers: {
          'content-type': 'application/json',
        },
        body: JSON.stringify({
          name,
          url,
        }),
      });

      const json = await resp.json();

      if (resp.status === 200) {
        dispatch(setMentatStatus(json));
      } else {
        if (isRosettaError(json))
          dispatch(addError(json, 'Adding network to backend'));
      }
    } catch (err) {
      console.error('unhandled error on network click', err);
    }
  }, [dispatch, netId, servers, url]);

  if (!hasBackend) return null;

  return (
    <NetworkNavStyle>
      {networks.length === 0 ? (
        <i>
          <I18n name="components.network_nav.no_networks_label" />
        </i>
      ) : (
        <>
          {networks.map(k => (
            <div
              key={k}
              className={'network ' + (endpoint === k ? 'active' : '')}
              onClick={onNetworkClicked(k)}
            >
              <img
                src={getCryptoIcon(servers[k].name)}
                className="crypto-icon"
              />
              {servers[k].name}
            </div>
          ))}
        </>
      )}
      <div className="flex" />
      {!(endpoint in servers) && netId && (
        <div className="button network" onClick={onAddNetworkClicked}>
          <FaPlus />
          <I18n name="components.network_nav.add_network_button" />
        </div>
      )}
    </NetworkNavStyle>
  );
};
